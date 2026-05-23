using System.Diagnostics;
using System.Text;

namespace Booth2RSS.Classes.Objects
{
    public class BoothStore
    {
        public string Name { get; private set; }
        public string Nickname { get; private set; }
        public string Description { get; private set; }
        public Uri Url { get; private set; }
        public Uri IconUrl { get; private set; }
        public BoothItem[] Items { get; private set; }

        public BoothStore(string name, string nickname, string description, Uri url, Uri iconUrl, BoothItem[] items)
        {
            Name = name;
            Nickname = nickname;
            Description = description;
            Url = url;
            IconUrl = iconUrl;
            Items = items;
        }

        public string AsRss(bool filterUnavailable=false, bool filterNSFW=true, bool vrcOnly=false, double ttl = 0)
        {
            StringBuilder stringBuilder = new();

            stringBuilder.Append("<?xml version=\"1.0\" encoding=\"UTF-8\" ?>");
            stringBuilder.Append("<rss version=\"2.0\">");
            stringBuilder.Append("<channel>");
            stringBuilder.Append($"<title>{Name}</title>");
            stringBuilder.Append($"<link>{Url}</link>");
            stringBuilder.Append($"<description>{Description}</description>");
            stringBuilder.Append("<generator>Booth2RSS</generator>");
            stringBuilder.Append($"<image>");
            stringBuilder.Append($"<url>{IconUrl}</url>");
            stringBuilder.Append($"<title>{Name}</title>");
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

                    if (vrcOnly && !item.IsVrchat)
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
