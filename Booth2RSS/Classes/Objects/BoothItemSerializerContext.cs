using Booth2RSS.Classes.Objects;
using System.Text.Json.Serialization;

namespace Booth2RSS.Classes
{
    [JsonSourceGenerationOptions(WriteIndented = true)]
    [JsonSerializable(typeof(BoothItem))]
    internal partial class BoothItemSerializerContext : JsonSerializerContext { }
}
