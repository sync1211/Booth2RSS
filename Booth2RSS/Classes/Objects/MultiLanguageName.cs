using System.Text.Json.Serialization;

namespace Booth2RSS.Classes.Objects
{
    public struct MultiLanguageName
    {
        [JsonPropertyName("ja")]
        public string Ja { get; private set; }
        [JsonPropertyName("en")]
        public string En { get; private set; }

        public MultiLanguageName(string name)
        {
            En = name;
            Ja = String.Empty;
        }

        [JsonConstructor]
        public MultiLanguageName(string ja, string en)
        {
            Ja = ja;
            En = en;
        }
    }
}
