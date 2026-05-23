using Booth2RSS.Classes.Objects;
using System.Net;
using System.Text;
using System.Text.Json;

namespace Booth2RSS.Classes
{
    public partial class BoothClient : IDisposable
    {
        public int LastError;
        public string LastErrorText = String.Empty;
        private readonly HttpClient httpClient;

        // Store
        private const string STORE_NAME_START="<span class=\"shop-name-label display_title\">";
        private const string STORE_NAME_END = "</span>";

        private const string STORE_NICK_START = "<a class=\"nav\" title=\"Home\" href=\"/\">";
        private const string STORE_NICK_END = "</a>";

        private const string STORE_DESC_START = "<div class=\"description\"><div class=\"booth-description\"><div class=\"autolink u-mb-[0-9]+\"><div>";
        private const string STORE_DESC_END = "</div>";

        private const string STORE_ICON_START = "<div class=\"avatar-image\" style=\"background-image: url(";
        private const string STORE_ICON_END = ")";

        private const string LAST_PAGE_START_STRING = "<a class=\"nav-item last-page\" href=\"/items?";
        private const string LAST_PAGE_SKIP = "page=";
        private const string CONTENT_END_STRING = "class=\"nav-item last-page\"";
        private const string CONTENT_START_STRING = "<body";

        // Item
        private const string ITEM_DATA_START = "data-item=\"";
        private const string ITEM_DATA_END = "\"";

        // Search
        private const string SEARCH_TITLE_START = "<h1 class=\"mb-0 u-tpg-title2 font-default-family font-normal\"><div class=\"text-text-default inline-block\">";
        private const string SEARCH_TITLE_END = "</div>";

        // Items (search)
        private const string ITEM_START = "<li class=\"item-card l-card";
        private const string ITEM_END = "</li>";

        private const string ITEM_ID_START = "data-item-id=\"";
        private const string ITEM_ID_END = "\"";
        private const string ITEM_THUMBNAIL_START = "<a target=\"_self\" class=\"js-thumbnail-image item-card__thumbnail-image\" data-tracking=\"click_item\" data-original=\"";
        private const string ITEM_THUMBNAIL_END = "\"";
        private const string ITEM_NAME_START = "data-product-name=\"";
        private const string ITEM_NAME_END = "\"";
        private const string ITEM_PRICE_START = "data-product-price=\"";
        private const string ITEM_PRICE_END = "\"";
        private const string ITEM_CATEGORY_START = "<a class=\"item-card__category-anchor nav-reverse\" data-product-list=\"from market_search_items via search_result_multiline to category_index\" data-tracking=\"click\" href=\"";
        private const string ITEM_CATEGORY_SEPARATOR = "\">";
        private const string ITEM_CATEGORY_END = "</a>";

        private const string ITEM_ADULT_FLAG = "<div class=\"badge adult\">Adult</div>";
        private const string ITEM_EOS_FLAG = "TODO: Implement me!";
        private const string ITEM_SOLD_OUT_FLAG = "<div class=\"badge empty-stock\">Out of Stock</div>";
        private const string ITEM_VRCHAT_FLAG = "<img alt=\"VRChat\" width=\"48px\" src=\"https://asset.booth.pm/static-images/shops/badges/vrchat.png\">";

        public string AcceptedLanguage
        {
            get
            {
                return httpClient.DefaultRequestHeaders.AcceptLanguage.FirstOrDefault()?.Value ?? " < None>";
            }
            set
            {
                httpClient.DefaultRequestHeaders.AcceptLanguage.Clear();
                httpClient.DefaultRequestHeaders.Add("Accept-Language", value);
            }
        }

        public BoothClient()
        {
            HttpClientHandler handler = new()
            {
                UseCookies = false
            };
            httpClient = new HttpClient(handler);
            AcceptedLanguage = "en-US";
		}

        private static BoothSearch CreateSearchFromContent(string content, Uri uri, BoothItem[] items)
        {
            string title = GetValueBetweenSnippets(content, SEARCH_TITLE_START, SEARCH_TITLE_END)
                ?? "(Parse error)";

            return new BoothSearch(title, uri, new Uri("https://booth.pm/favicon.ico"), items);
        }

        private BoothStore CreateStoreFromContent(string content, Uri uri, BoothItem[] items)
        {
            string nickname = GetValueBetweenSnippets(content, STORE_NICK_START, STORE_NICK_END)
                ?? "(Parse error)";
            string name = GetValueBetweenSnippets(content, STORE_NAME_START, STORE_NAME_END)
                ?? nickname;
            string description = GetValueBetweenSnippets(content, STORE_DESC_START, STORE_DESC_END)
                ?? "(Not found!)";
            string iconUrl = GetValueBetweenSnippets(content, STORE_ICON_START, STORE_ICON_END)
                ?? string.Empty;

            return new BoothStore(name, nickname, description, uri, new Uri(iconUrl), items);
        }

        private static string? GetValueBetweenSnippets(string content, string start, string end)
        {
            return GetValueBetweenSnippets(content, start, end, 0, out _);
        }

        private static string? GetValueBetweenSnippets(string content, string start, string end, int inOffset, out int outOffset)
        {
            outOffset = inOffset;

            int startIndex = content.IndexOf(start, inOffset);
            if (startIndex == -1)
            {
                return null;
            }
            int valueStartIndex = startIndex + start.Length;

            int endIndex = content.IndexOf(end, valueStartIndex);
            if (endIndex == -1)
            {
                return null;
            }

            outOffset = endIndex;
            return content[valueStartIndex..endIndex];
        }

        public static int? GetPageCountFromContent(string content)
        {
            string? pageString = GetValueBetweenSnippets(content, LAST_PAGE_START_STRING, "\"");
            if (pageString == null)
            {
                return null;
            }

            int pageNumIndex = pageString.IndexOf(LAST_PAGE_SKIP);
            if (pageNumIndex != -1)
            {
                pageString = pageString.Substring(pageNumIndex);
            }

            int paramIndex = pageString.IndexOf('&');
            if (paramIndex != -1)
            {
                pageString = pageString[..paramIndex];
            }

            if (int.TryParse(pageString, out int pageCount))
            {
                return pageCount;
            }

            return null;
        }

        public async Task<BoothSearch?> GetSearch(Uri uri, int maxPages = 100, bool unblurNSFW = false, CancellationToken cancellationToken = default)
        {
            string uriString = uri.ToString().TrimEnd('/');
            //TODO: Adjust for search
            //if (!(uriString.Contains("items") || uriString.Contains("item_lists")))
            //{
            //    uriString += "/items";
            //}

            //TODO: Deduplicate this block
            int? pageCount = null;

            int i = 1;
            List<BoothItem> items = [];
            while (true)
            {
                Console.Write($"Fetching page {i}/{pageCount?.ToString() ?? "?"} - ");
                HttpRequestMessage requestMessage = new(HttpMethod.Get, $"{uriString}?page={i}");

                if (unblurNSFW)
                {
                    requestMessage.Headers.Add("Cookie", "adult=t");
                }
                HttpResponseMessage response = await httpClient.SendAsync(requestMessage, cancellationToken);
                requestMessage.Dispose();


                // Check status
                Console.Write($"{response.StatusCode} - ");
                if (response.StatusCode != HttpStatusCode.OK)
                {
                    LastError = (int)response.StatusCode;
                    LastErrorText = await response.Content.ReadAsStringAsync(cancellationToken);

                    return null;
                }

                // Get Content
                string content = await ReadPageContent(response.Content, cancellationToken);

                // Detect number of pages
                pageCount ??= GetPageCountFromContent(content);

                // Parse content
                BoothItem[] newItems = GetItemsFromSearchContent(content);

                Console.WriteLine(newItems.Length);
                items.AddRange(newItems);

                // Exit condition
                if (newItems.Length == 0 || i >= maxPages || i >= pageCount)
                {
                    Console.WriteLine("Last page reached.");

                    return CreateSearchFromContent(content, uri, [.. items]);
                }

                i++;
            }
        }

        public async Task<BoothStore?> GetStore(Uri uri, int maxPages = 100, bool unblurNSFW=false, CancellationToken cancellationToken = default)
        {
            string uriString = uri.ToString().TrimEnd('/');
            if (!(uriString.Contains("items") || uriString.Contains("item_lists")))
            {
                uriString += "/items";
            }

            int? pageCount = null;

            int i = 1;
            List<BoothItem> items = [];
            while (true)
            {
                Console.Write($"Fetching page {i}/{pageCount?.ToString() ?? "?"} - ");
                HttpRequestMessage requestMessage = new(HttpMethod.Get, $"{uriString}?page={i}");

                if (unblurNSFW)
                {
                    requestMessage.Headers.Add("Cookie", "adult=t");
                }
                HttpResponseMessage response = await httpClient.SendAsync(requestMessage, cancellationToken);
                requestMessage.Dispose();


                // Check status
                Console.Write($"{response.StatusCode} - ");
                if (response.StatusCode != HttpStatusCode.OK)
                {
                    LastError = (int)response.StatusCode;
                    LastErrorText = await response.Content.ReadAsStringAsync(cancellationToken);

                    return null;
                }

                // Get Content
                string content = await ReadPageContent(response.Content, cancellationToken);

                // Detect number of pages
                pageCount ??= GetPageCountFromContent(content);

                // Parse content
                BoothItem[] newItems = GetItemsFromStoreContent(content);

                Console.WriteLine(newItems.Length);
                items.AddRange(newItems);

                // Exit condition
                if (newItems.Length == 0 || i >= maxPages || i >= pageCount)
                {
                    Console.WriteLine("Last page reached.");

                    return CreateStoreFromContent(content, uri, [.. items]);
                }

                i++;
            }
        }

        private static async Task<string> ReadPageContent(HttpContent content, CancellationToken cancellationToken=default)
        {
            StringBuilder contentBuilder = new();
            await using (Stream stream = await content.ReadAsStreamAsync(cancellationToken))
            using (StreamReader reader = new(stream))
            {
                // Read until the store page starts
                while (true)
                {
                    string? contentLine = await reader.ReadLineAsync(cancellationToken);
                    if (contentLine == null)
                    {
                        return String.Empty;
                    }

                    if (contentLine.Contains(CONTENT_START_STRING))
                    {
                        contentBuilder.AppendLine(contentLine);
                        break;
                    }
                }

                while (true)
                {
                    string? contentLine = await reader.ReadLineAsync(cancellationToken);

                    if (contentLine == null)
                    {
                        break;
                    }

                    contentBuilder.AppendLine(contentLine);
                    if (contentLine.Contains(CONTENT_END_STRING))
                    {
                        break;
                    }
                }
            }

            return contentBuilder.ToString();
        }

        private static BoothItem[] GetItemsFromStoreContent(string content)
        {
            string[] itemData = GetItemDataFromStoreContent(content);

            BoothItem[] items = new BoothItem[itemData.Length];
            for (int i = 0; i < itemData.Length; i++)
            {
                BoothItem? item = JsonSerializer.Deserialize(itemData[i], BoothItemSerializerContext.Default.BoothItem);
                items[i] = item ?? new BoothItem(0, "ERROR PARSING ITEM DATA!");
            }

            return items;
        }

        private static BoothCategory? GetCategoryFromSearchContent(string content)
        {
            string? categoryString = GetValueBetweenSnippets(content, ITEM_CATEGORY_START, ITEM_CATEGORY_END);
            if (categoryString == null)
            {
                return null;
            }

            int categorySeparatorIndex = categoryString.IndexOf(ITEM_CATEGORY_SEPARATOR);
            if (categorySeparatorIndex == -1)
            {
                return null;
            }
            string categoryName = categoryString[(categorySeparatorIndex + ITEM_CATEGORY_SEPARATOR.Length)..];
            string categoryLink = categoryString[..categorySeparatorIndex];


            return new BoothCategory(categoryName, new Uri(categoryLink));
        }

        private static BoothItem[] GetItemsFromSearchContent(string content)
        {
            List<BoothItem> items = [];
            int offset = 0;

            while (true)
            {
                int itemStartIndex = content.IndexOf(ITEM_START, offset);
                if (itemStartIndex == -1)
                {
                    break;
                }

                int itemEndIndex = content.IndexOf(ITEM_END, itemStartIndex);
                if (itemEndIndex == -1)
                {
                    itemEndIndex = content.Length;
                }

                string itemDataString = content[itemStartIndex..itemEndIndex];
                offset = itemEndIndex;

                // ID
                string? idString = GetValueBetweenSnippets(itemDataString, ITEM_ID_START, ITEM_ID_END);
                if (idString == null)
                {
                    break;
                }

                if (!int.TryParse(idString, out int id))
                {
                    Console.WriteLine("Unable to parse item ID!");
                    continue;
                }

                // Item data
                string thumbnail = GetValueBetweenSnippets(itemDataString, ITEM_THUMBNAIL_START, ITEM_THUMBNAIL_END)
                    ?? "Not found!";
                string name = GetValueBetweenSnippets(itemDataString, ITEM_NAME_START, ITEM_NAME_END)
                    ?? "Not found!";
                string price = GetValueBetweenSnippets(itemDataString, ITEM_PRICE_START, ITEM_PRICE_END)
                    ?? "Not found!";

                BoothCategory category = GetCategoryFromSearchContent(itemDataString)
                    ?? new BoothCategory("Uncategorized", new Uri("https://booth.pm/"));

                bool isAdult = itemDataString.Contains(ITEM_ADULT_FLAG);
                bool isEndOfSale = itemDataString.Contains(ITEM_EOS_FLAG);
                bool isSoldOut = itemDataString.Contains(ITEM_SOLD_OUT_FLAG);
                bool isVRChat = itemDataString.Contains(ITEM_VRCHAT_FLAG);

                Uri link = new($"https://booth.pm/en/items/{id}");

                items.Add(new BoothItem(0, name, category, isAdult, isEndOfSale, false, isSoldOut, isVRChat, null, price, [thumbnail], link.OriginalString));
            }

            return [.. items];
        }

        private static string[] GetItemDataFromStoreContent(string content)
        {
            List<string> itemData = [];
            int offset = 0;

            while (true)
            {
                string? itemDataString = GetValueBetweenSnippets(content, ITEM_DATA_START, ITEM_DATA_END, offset, out offset);
                if (itemDataString == null)
                {
                    break;
                }

                itemData.Add(itemDataString.Replace("&quot;", "\"").Trim('\\'));
            }

            return [.. itemData];
        }

        public void Dispose()
        {
            httpClient.Dispose();
            GC.SuppressFinalize(this);
        }
    }
}
