using System.Text;

namespace Booth2RSS.Classes.Objects
{
    public class BoothSearch
    {
        public string Title { get; private set; }
        public Uri Url { get; private set; }
        public Uri IconUrl { get; private set; }
        public BoothItem[] Items { get; private set; }

        public BoothSearch(string title, Uri url, Uri iconUrl, BoothItem[] items)
        {
            Title = title;
            Url = url;
            IconUrl = iconUrl;
            Items = items;
        }

        public string AsRss(bool filterUnavailable=false, bool filterNSFW=true, double ttl = 0)
        {
            StringBuilder stringBuilder = new();

            stringBuilder.Append("<?xml version=\"1.0\" encoding=\"UTF-8\" ?>");
            stringBuilder.Append("<rss version=\"2.0\">");
            stringBuilder.Append("<channel>");
            stringBuilder.Append($"<title>Search: {Title}</title>");
            stringBuilder.Append($"<link>{Url}</link>");
            stringBuilder.Append("<generator>Booth2RSS</generator>");
            stringBuilder.Append($"<image>");
            stringBuilder.Append($"<url>{IconUrl}</url>");
            stringBuilder.Append($"<title>Search: {Title}</title>");
            stringBuilder.Append($"<link>{IconUrl}</link>");
            stringBuilder.Append($"</image>");
            stringBuilder.Append("<category>Store</category>");
            stringBuilder.Append($"<ttl>{ttl}</ttl>");

            for (int i = 0; i < Items.Length; i++)
                {
                    BoothItem item = Items[i];
                    if (filterUnavailable && (item.IsSoldOut || item.IsEndOfSale))
                    {
                        continue;
                    }

                    if (filterNSFW && item.IsAdult)
                    {
                        continue;
                    }

                    stringBuilder.Append(Items[i].AsRss());
                }
            stringBuilder.Append("</channel>");
            stringBuilder.Append("</rss>");

            return stringBuilder.ToString();
        }
    }
}
