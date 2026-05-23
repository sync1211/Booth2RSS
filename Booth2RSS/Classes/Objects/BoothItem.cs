using System.Text;
using System.Text.Json.Serialization;

namespace Booth2RSS.Classes.Objects
{
    public class BoothItem
    {
        [JsonPropertyName("id")]
        public int Id { get; private set; }
        [JsonPropertyName("name")]
        public string Name { get; private set; }
        [JsonPropertyName("category")]
        public BoothCategory Category { get; private set; }
        [JsonPropertyName("is_adult")]
        public bool IsAdult { get; private set; }
        [JsonPropertyName("is_end_of_sale")]
        public bool IsEndOfSale { get; private set; }
        [JsonPropertyName("is_placeholder")]
        public bool IsPlaceholder { get; private set; }
        [JsonPropertyName("is_sold_out")]
        public bool IsSoldOut { get; private set; }
        [JsonPropertyName("is_vrchat")]
        public bool IsVrchat { get; private set; }
        [JsonPropertyName("minimum_stock")]
        public int? MinimumStock { get; private set; }
        [JsonPropertyName("price")]
        public string Price { get; private set; }
        [JsonPropertyName("thumbnail_image_urls")]
        public string[] ThumbnailImageUrls { get; private set; }
        [JsonPropertyName("shop_item_url")]
        public string Url { get; private set; }

	[JsonIgnore]
	public string Description
	{
		get
		{
			// Content tags
			StringBuilder contentTags = new();
			if (IsAdult)
			{
				contentTags.Append("[ADULT CONTENT]");
			}

			if (IsVrchat)
			{
				contentTags.AppendJoin(' ', "[VRChat]");
			}

			if (IsPlaceholder)
			{
				contentTags.AppendJoin(' ', "[PLACEHOLDER]");
			}

			// Description
			string description = $"Category: {Category.Name}{Environment.NewLine}Price: {Price}";
			if (contentTags.Length > 0)
			{
				description = $"{contentTags}{Environment.NewLine}{description}";
			}

			if (IsSoldOut)
			{
				description += " (Sold Out)";
			}

			if (IsEndOfSale)
			{
				description += " (EndOfSale)";
			}

			return description;
		}
	}

        public BoothItem(int id, string name)
        {
            Id = id;
            Name = name;
            Category = new BoothCategory("Uncategorized", new Uri("https://booth.pm/"));
            Url = "https://booth.pm/";
            Price = String.Empty;
            ThumbnailImageUrls = [];
        }

        [JsonConstructor]
        public BoothItem(int id, string name, BoothCategory category, bool isAdult, bool isEndOfSale, bool isPlaceholder, bool isSoldOut, bool isVrchat, int? minimumStock, string price, string[] thumbnailImageUrls, string url)
        {
            Id = id;
            Name = name;
            Category = category;
            IsAdult = isAdult;
            IsEndOfSale = isEndOfSale;
            IsPlaceholder = isPlaceholder;
            IsSoldOut = isSoldOut;
            IsVrchat = isVrchat;
            MinimumStock = minimumStock;
            Price = price;
            ThumbnailImageUrls = thumbnailImageUrls;
            Url = url;
        }

        public override string ToString()
        {
            return $"{Name} - {Price} ({Url})";
        }

        public string AsRss()
        {
            string displayName = IsAdult
                ? $"🔞 {Name}"
                : Name;

            string thumbnailUrl = ThumbnailImageUrls[0];

			// ID of the current item state
			// Any change in price or availability will be treated as a new entry by RSS readers
			string stateID = $"{Id};{(IsSoldOut ? "SO" : String.Empty)}{(IsEndOfSale ? "EOS" : String.Empty)}{(IsPlaceholder ? "PH" :String.Empty)};{Price}";

            StringBuilder stringBuilder = new();

			stringBuilder.Append("<item>");
			stringBuilder.Append($"<title>{displayName}</title>");
			stringBuilder.Append($"<link>{Url}</link>");
			stringBuilder.Append($"<description>{Description}");
			stringBuilder.Append($"<![CDATA[ <img src=\"{thumbnailUrl}\" alt=\"{Name}\" title=\"\"/> ]]>");
			stringBuilder.Append("</description>");
			stringBuilder.Append($"<guid>{stateID}</guid>");
			stringBuilder.Append($"<category>{Category.Name}</category>");
			stringBuilder.Append("</item>");

            return stringBuilder.ToString();
        }
    }
}
