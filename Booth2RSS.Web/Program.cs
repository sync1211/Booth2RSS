using Booth2RSS.Classes;
using Booth2RSS.Classes.Objects;
using Booth2RSS.Web.Classes;

namespace Booth2RSS.Web
{
    public static class Program
    {
        private static readonly TimeSpan CacheDuration = TimeSpan.FromMinutes(15);
        private static readonly StringCache cache = new(CacheDuration);

        public static void Main(string[] args)
        {

            WebApplicationBuilder builder = WebApplication.CreateBuilder(args);
            WebApplication app = builder.Build();

            using (BoothClient client = new())
            {
                app.MapGet(
                    "/booth2rss/store",
                    async (
                        string? url = null,
                        int limit = 10,
                        bool filter_unavailable = true,
                        bool unblurNSFW = false,
                        bool allowNSFW = false,
                        bool vrcOnly = false
                    ) => await GetStoreFeed(client, url, limit, filter_unavailable, unblurNSFW, allowNSFW, vrcOnly)
                ).WithName("GetStore");

                //app.MapGet(
                //    "/booth2rss/search",
                //    async (
                //        string? url = null,
                //        int limit = 10,
                //        bool filter_unavailable = true,
                //        bool unblurNSFW = false,
                //        bool allowNSFW = false
                //    ) => await GetSearchFeed(client, url, limit, filter_unavailable, unblurNSFW, allowNSFW)
                //).WithName("GetSearch");

                app.Run();
            }
        }

        private static async Task<string> GetStoreFeed(BoothClient client, string? storeUrl, int limit, bool filterUnavailable, bool unblurNSFW, bool allowNSFW, bool vrcOnly, CancellationToken cancellationToken=default)
        {
            if (storeUrl == null)
            {
                return "The parameter 'URL' is required!";
            }

            if (!storeUrl.StartsWith("https://"))
            {
                storeUrl = $"https://{storeUrl}";
            }

            if (storeUrl.IndexOf("booth.pm") < storeUrl.IndexOf('/'))
            {
                return "Only URLs hosted on booth.pm are supported!";
            }

            string key = $"L{limit}U{unblurNSFW}N{allowNSFW}S{filterUnavailable};{storeUrl}";
            if (cache.GetCachedValue(key) is string cachedValue)
            {
                return cachedValue;
            }

            try
            {
                Uri uri = new(storeUrl);
                BoothStore? store = await client.GetStore(uri, limit, unblurNSFW, cancellationToken);
                if (store == null)
                {
                    return $"Error: {client.LastErrorText}";
                }

                string rss = store.AsRss(filterUnavailable, !allowNSFW, vrcOnly, CacheDuration.TotalMinutes);
                cache.AddToCache(key, rss);
                return rss;
            }
            catch (Exception ex)
            {
                Console.WriteLine($"Exception: {ex.Message}{Environment.NewLine}{ex.StackTrace}");
                return "Something didn't work.";
            }
        }

        //private static async Task<string> GetSearchFeed(BoothClient client, string? searchUrl, int limit, bool filterUnavailable, bool unblurNSFW, bool allowNSFW, CancellationToken cancellationToken = default)
        //{
        //    if (searchUrl == null)
        //    {
        //        return "The parameter 'URL' is required!";
        //    }

        //    if (!searchUrl.StartsWith("https://"))
        //    {
        //        searchUrl = $"https://{searchUrl}";
        //    }

        //    if (searchUrl.IndexOf("booth.pm") < searchUrl.IndexOf('/'))
        //    {
        //        return "Only URLs hosted on booth.pm are supported!";
        //    }

        //    string key = $"L{limit}U{unblurNSFW}N{allowNSFW}S{filterUnavailable};{searchUrl}";
        //    if (cache.GetCachedValue(key) is string cachedValue)
        //    {
        //        return cachedValue;
        //    }

        //    try
        //    {
        //        Uri uri = new(searchUrl);
        //        BoothSearch? search = await client.GetSearch(uri, limit, unblurNSFW, cancellationToken);
        //        if (search == null)
        //        {
        //            return $"Error: {client.LastErrorText}";
        //        }

        //        string rss = search.AsRss(filterUnavailable, !allowNSFW, CacheDuration.TotalMinutes);
        //        cache.AddToCache(key, rss);
        //        return rss;
        //    }
        //    catch (Exception ex)
        //    {
        //        Console.WriteLine($"Exception: {ex.Message}{Environment.NewLine}{ex.StackTrace}");
        //        return "Something didn't work.";
        //    }
        //}
    }
}
