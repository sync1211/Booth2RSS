using System.Text.Json.Serialization;

namespace Booth2RSS.Classes.Objects
{
    public struct BoothCategory
    {
        [JsonPropertyName("name")]
        public MultiLanguageName ItemName { get; private set; }
        [JsonPropertyName("url")]
        public Uri Url { get; private set; }
        [JsonIgnore]
        public string Name
        {
            get {
                return ItemName.En;
            }
        }

        [JsonConstructor]
        public BoothCategory(MultiLanguageName itemName, Uri url)
        {
            ItemName = itemName;
            Url = url;
        }

        public BoothCategory(string name, Uri url)
        {
            ItemName = new MultiLanguageName(name);
            Url = url;
        }
    }
}
