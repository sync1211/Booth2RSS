using Booth2RSS.Classes;
using Booth2RSS.Classes.Objects;

namespace Booth2RSS.CLI
{
    public static class Program
    {
        public static void Main(string[] args)
        {
            if (args.Length < 2)
            {
                Console.WriteLine("Incorrect number of arguments provided!");
                return;
            }

            // Argument parsing
            for (int i = 1; i < args.Length; i++)
            {
                args[i] = args[i].Trim().ToLower();
            }

            bool unblurNSFW = args.Contains("--unblur-nsfw");
            bool allowNSFW = args.Contains("--allow-nsfw");
            bool filterUnavailable = !args.Contains("--include-unavailable");
            bool vrcOnly = !args.Contains("--vrc-only");

            // Get data
            string mode = args[0];
            Uri uri = new(args[1]);

            switch (mode)
            {
                case "store":
                    PrintStoreRSS(uri, unblurNSFW, allowNSFW, filterUnavailable, vrcOnly);
                    break;
                case "search":
                    PrintSearchRSS(uri, unblurNSFW, allowNSFW, filterUnavailable);
                    break;
                default:
                    break;
            }
        }

        private static void PrintStoreRSS(Uri url, bool unblurNSFW, bool allowNSFW, bool filterUnavailable, bool vrcOnly)
        {
            using (BoothClient client = new())
            {
                BoothStore? store = client.GetStore(url, unblurNSFW: unblurNSFW).Result;
                if (store == null)
                {
                    Console.WriteLine("Failed to get store.");
                    Console.WriteLine(client.LastErrorText);
                    return;
                }
                Console.WriteLine(store.AsRss(filterUnavailable, !allowNSFW, vrcOnly: vrcOnly));
            }
        }

        private static void PrintSearchRSS(Uri url, bool unblurNSFW, bool allowNSFW, bool filterUnavailable)
        {
            using (BoothClient client = new())
            {
                BoothSearch? search = client.GetSearch(url, unblurNSFW: unblurNSFW).Result;
                if (search == null)
                {
                    Console.WriteLine("Failed to get search page.");
                    Console.WriteLine(client.LastErrorText);
                    return;
                }
                Console.WriteLine(search.AsRss(filterUnavailable, !allowNSFW));
            }
        }
    }
}
