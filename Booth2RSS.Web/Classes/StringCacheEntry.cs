namespace Booth2RSS.Web.Classes
{
    public struct StringCacheEntry
    {
        public string Value;
        public DateTime CreatedAt;
        public TimeSpan Age
        {
            get
            {
                return DateTime.UtcNow - CreatedAt;
            }
        }

        public StringCacheEntry(string value)
        {
            Value = value;
            CreatedAt = DateTime.UtcNow;
        }
    }
}
