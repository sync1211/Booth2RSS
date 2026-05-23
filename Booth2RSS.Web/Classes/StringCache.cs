namespace Booth2RSS.Web.Classes
{
    public class StringCache
    {
        public TimeSpan MaxAge = TimeSpan.FromMinutes(15);
        public int MaxSize;
        private readonly Dictionary<string, StringCacheEntry> cacheDictionary = [];

        public StringCache(TimeSpan maxAge, int maxSize = 100)
        {
            MaxAge = maxAge;
            MaxSize = maxSize;
        }

        public void AddToCache(string key, string value)
        {
            if (key == null)
            {
                Console.WriteLine("KEY MUST NOT BE NULL!");
            }
            int i = CleanupExpired();
            if (i > 0)
            {
                Console.WriteLine($"Cleaned up {i} expired cache entries");
            }
            while (cacheDictionary.Count >= MaxSize)
            {
                RemoveOldest();
            }

            cacheDictionary[key] = new StringCacheEntry(value);
        }

        public string? GetCachedValue(string key)
        {
            if (!cacheDictionary.TryGetValue(key, out StringCacheEntry entry))
            {
                return null;
            }

            if (entry.Age > MaxAge)
            {
                cacheDictionary.Remove(key);
                return null;
            }

            return entry.Value;
        }

        public bool Remove(string key)
        {
            return cacheDictionary.Remove(key);
        }

        public bool RemoveOldest()
        {
            KeyValuePair<string, StringCacheEntry>[] kvps = cacheDictionary.ToArray();

            string oldestKey = String.Empty;
            TimeSpan oldestValue = TimeSpan.FromSeconds(0);

            for (int i = 0; i < cacheDictionary.Count; i++)
            {
                KeyValuePair<string, StringCacheEntry> kvp = kvps[i];
                string key = kvp.Key;
                StringCacheEntry entry = kvp.Value;
                if (entry.Age > oldestValue)
                {
                    oldestValue = entry.Age;
                    oldestKey = key;
                }
            }

            if (!String.IsNullOrEmpty(oldestKey))
            {
                return Remove(oldestKey);
            }

            return false;
        }

        public int CleanupExpired()
        {
            int delCount = 0;
            
            KeyValuePair<string, StringCacheEntry>[] kvps = cacheDictionary.ToArray();
            for (int i = 0; i < cacheDictionary.Count; i++)
            {
                KeyValuePair<string, StringCacheEntry> kvp = kvps[i];
                string key = kvp.Key;
                StringCacheEntry entry = kvp.Value;
                
                if (entry.Age > MaxAge)
                {
                    if (Remove(key))
                    {
                        i++;
                    }
                }
            }
            return delCount;
        }

        public void Clear()
        {
            cacheDictionary.Clear();
        }
    }
}
