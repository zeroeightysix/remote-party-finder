using System;
using System.Collections.Generic;
using System.IO;
using System.Linq;
using System.Text;
using Lumina;
using Lumina.Data;
using Lumina.Excel;
using Lumina.Excel.Sheets;
using Lumina.Text;
using Lumina.Text.ReadOnly;
using Pidgin;

namespace SourceGenerator;

internal class Program
{
    private static void Main(string[] args)
    {
        if (args.Length < 2)
        {
            Console.WriteLine($"Usage: SourceGenerator <sqpack dir> <out dir>");
            return;
        }

        var data = new Dictionary<Language, GameData>(4);
        foreach (var lang in Languages.Keys)
        {
            data[lang] = new GameData(args[0], new LuminaOptions
            {
                PanicOnSheetChecksumMismatch = false,
                DefaultExcelLanguage = lang,
            });
        }

        var prog = new Program(data);
        var outPath = args[1];
        File.WriteAllText(Path.Join(outPath, "duties.rs"), prog.GenerateDuties());
        File.WriteAllText(Path.Join(outPath, "jobs.rs"), prog.GenerateJobs());
        File.WriteAllText(Path.Join(outPath, "roulettes.rs"), prog.GenerateRoulettes());
        File.WriteAllText(Path.Join(outPath, "worlds.rs"), prog.GenerateWorlds());
        File.WriteAllText(Path.Join(outPath, "territory_names.rs"), prog.GenerateTerritoryNames());
        File.WriteAllText(Path.Join(outPath, "auto_translate.rs"), prog.GenerateAutoTranslate());
        File.WriteAllText(Path.Join(outPath, "treasure_maps.rs"), prog.GenerateTreasureMaps());
    }

    private Dictionary<Language, GameData> Data { get; }

    private Program(Dictionary<Language, GameData> data)
    {
        this.Data = data;
    }

    private static StringBuilder DefaultHeader(bool localisedText = false)
    {
        var sb = new StringBuilder("use std::collections::HashMap;\n");

        if (localisedText)
        {
            sb.Append("use super::LocalisedText;\n");
        }

        return sb;
    }

    private static readonly Dictionary<Language, string> Languages = new()
    {
        [Language.English] = "en",
        [Language.Japanese] = "ja",
        [Language.German] = "de",
        [Language.French] = "fr",
    };

    private static string GetFlagName(string jobName)
    {
        // Convert job name to flag format: add underscores before capital letters and make uppercase
        var result = System.Text.RegularExpressions.Regex.Replace(jobName, "([a-z])([A-Z])", "$1_$2");
        return result.ToUpperInvariant();
    }

    private string? GetLocalisedStruct<T>(uint rowId, Func<T, ReadOnlySeString?> nameFunc, uint indent = 0,
        bool capitalise = false) where T : struct, IExcelRow<T>
    {
        var def = this.Data[Language.English].GetExcelSheet<T>()!.GetRow(rowId)!;
        var defName = nameFunc(def)?.ExtractText();
        if (string.IsNullOrEmpty(defName))
        {
            return null;
        }

        var sb = new StringBuilder();

        sb.Append("LocalisedText {\n");

        foreach (var (language, key) in Languages)
        {
            var row = this.Data[language].GetExcelSheet<T>(language)?.GetRow(rowId);
            var name = row == null
                ? defName
                : nameFunc((T)row)?.ExtractText().Replace("\"", "\\\"").Replace(" ", "").Replace("­", "");
            name ??= defName;
            if (capitalise)
            {
                name = name[..1].ToUpperInvariant() + name[1..];
            }

            for (var i = 0; i < indent + 4; i++)
            {
                sb.Append(' ');
            }

            sb.Append($"{key}: \"{name}\",\n");
        }

        for (var i = 0; i < indent; i++)
        {
            sb.Append(' ');
        }

        sb.Append('}');

        return sb.ToString();
    }

    private string GenerateDuties()
    {
        var sb = DefaultHeader(true);
        sb.Append('\n');

        sb.Append("#[derive(Debug, Copy, Clone)]\n");
        sb.Append("pub struct DutyInfo {\n");
        sb.Append("    pub name: LocalisedText,\n");
        sb.Append("    pub high_end: bool,\n");
        sb.Append("    pub content_kind: ContentKind,\n");
        sb.Append("}\n\n");

        sb.Append("#[derive(Debug, Clone, Copy)]\n");
        sb.Append("#[allow(unused)]\n");
        sb.Append("#[repr(u32)]\n");
        sb.Append("pub enum ContentKind {\n");
        foreach (var kind in this.Data[Language.English].GetExcelSheet<ContentType>()!)
        {
            var name = kind.Name.ExtractText().Replace(" ", "").Replace("&", "");
            if (name.Length > 0)
            {
                sb.Append($"    {name} = {kind.RowId},\n");
            }
        }

        sb.Append("    Other(u32),\n");
        sb.Append("}\n\n");

        sb.Append("impl ContentKind {\n");
        sb.Append("    #[expect(unused)]\n");

        sb.Append("    fn from_u32(kind: u32) -> Self {\n");
        sb.Append("        match kind {\n");
        foreach (var kind in this.Data[Language.English].GetExcelSheet<ContentType>()!)
        {
            var name = kind.Name.ExtractText().Replace(" ", "").Replace("&", "");
            if (name.Length > 0)
            {
                sb.Append($"            {kind.RowId} => Self::{name},\n");
            }
        }

        sb.Append("            x => Self::Other(x),\n");
        sb.Append("        }\n");
        sb.Append("    }\n\n");

        sb.Append("    pub fn as_u32(self) -> u32 {\n");
        sb.Append("        match self {\n");
        foreach (var kind in this.Data[Language.English].GetExcelSheet<ContentType>()!)
        {
            var name = kind.Name.ExtractText().Replace(" ", "").Replace("&", "");
            if (name.Length > 0)
            {
                sb.Append($"            Self::{name} => {kind.RowId},\n");
            }
        }

        sb.Append("            Self::Other(x) => x,\n");
        sb.Append("        }\n");
        sb.Append("    }\n");

        sb.Append("}\n\n");

        sb.Append("lazy_static::lazy_static! {\n");
        sb.Append("    pub static ref DUTIES: HashMap<u32, DutyInfo> = maplit::hashmap! {\n");

        foreach (var cfc in this.Data[Language.English].GetExcelSheet<ContentFinderCondition>()!)
        {
            if (cfc.RowId == 0)
            {
                continue;
            }

            var name = this.GetLocalisedStruct<ContentFinderCondition>(cfc.RowId, row => row.Name, 12, true);
            if (name == null)
            {
                continue;
            }

            var highEnd = cfc.HighEndDuty ? "true" : "false";
            var contentType = cfc.ContentType.Value;
            var contentKind = contentType.Name.ExtractText().Replace(" ", "").Replace("&", "");
            if (string.IsNullOrEmpty(contentKind))
            {
                contentKind = $"Other({contentType.RowId})";
            }

            sb.Append($"        {cfc.RowId} => DutyInfo {{\n");
            sb.Append($"            name: {name},\n");
            sb.Append($"            high_end: {highEnd},\n");
            sb.Append($"            content_kind: ContentKind::{contentKind},\n");
            sb.Append("        },\n");
        }

        sb.Append("    };\n");
        sb.Append("}\n");

        return sb.ToString();
    }

    private string GenerateJobs()
    {
        var sb = DefaultHeader();
        sb.Append("use crate::listing::JobFlags;\n");
        sb.Append("use ffxiv_types::jobs::{Class, ClassJob, Job, NonCombatJob};\n\n");
        sb.Append("lazy_static::lazy_static! {\n");
        sb.Append("    pub static ref JOBS: HashMap<u32, ClassJob> = maplit::hashmap! {\n");

        foreach (var cj in this.Data[Language.English].GetExcelSheet<ClassJob>()!)
        {
            if (cj.RowId == 0)
            {
                continue;
            }

            var name = cj.NameEnglish.ExtractText().Replace(" ", "");
            if (name.Length <= 0)
            {
                continue;
            }

            // Determine job type for all jobs (including NonCombat)
            string value;
            if (cj.RowId >= 8 && cj.RowId <= 18)
            {
                // Crafters and gatherers (8-18)
                value = $"ClassJob::NonCombat(NonCombatJob::{name})";
            }
            else if (cj.RowId >= 1 && cj.RowId <= 7)
            {
                // Base combat classes (1-7)
                value = $"ClassJob::Class(Class::{name})";
            }
            else if (cj.RowId == 19)
            {
                // Paladin should be Job
                value = $"ClassJob::Job(Job::{name})";
            }
            else if (cj.RowId == 20)
            {
                // Monk should be Job
                value = $"ClassJob::Job(Job::{name})";
            }
            else if (cj.RowId == 24)
            {
                // WhiteMage should be Job
                value = $"ClassJob::Job(Job::{name})";
            }
            else if (cj.RowId == 25)
            {
                // BlackMage should be Job
                value = $"ClassJob::Job(Job::{name})";
            }
            else if (cj.RowId == 26)
            {
                // Arcanist should be Class
                value = $"ClassJob::Class(Class::{name})";
            }
            else if (cj.RowId == 29)
            {
                // Rogue should be Class
                value = $"ClassJob::Class(Class::{name})";
            }
            else if (cj.RowId == 36)
            {
                // Blue Mage should be Job
                value = $"ClassJob::Job(Job::{name})";
            }
            else if (cj.RowId == 43)
            {
                // Temporarily disable support for beastmaster.
                continue;
            }
            else
            {
                // All other combat jobs (21+ except the special cases above)
                value = $"ClassJob::Job(Job::{name})";
            }

            sb.Append($"        {cj.RowId} => {value},\n");
        }

        sb.Append("    };\n\n");
        sb.Append("    pub static ref JOBS_TO_FLAGS: HashMap<&'static str, JobFlags> = maplit::hashmap! {\n");

        foreach (var cj in this.Data[Language.English].GetExcelSheet<ClassJob>()!)
        {
            if (cj.RowId == 0)
            {
                continue;
            }

            var name = cj.NameEnglish.ExtractText().Replace(" ", "");
            if (name.Length <= 0)
            {
                continue;
            }

            // Skip NonCombat jobs (crafters, gatherers only)
            if (cj.RowId >= 8 && cj.RowId <= 18)
            {
                continue;
            }

            // Determine job type for combat jobs only (same logic as above)
            string classJobStr;
            string flagName;

            if (cj.RowId >= 1 && cj.RowId <= 7)
            {
                // Base combat classes (1-7)
                classJobStr = $"ClassJob::Class(Class::{name})";
                flagName = GetFlagName(name);
            }
            else if (cj.RowId == 19)
            {
                // Paladin should be Job
                classJobStr = $"ClassJob::Job(Job::{name})";
                flagName = GetFlagName(name);
            }
            else if (cj.RowId == 20)
            {
                // Monk should be Job
                classJobStr = $"ClassJob::Job(Job::{name})";
                flagName = GetFlagName(name);
            }
            else if (cj.RowId == 24)
            {
                // WhiteMage should be Job
                classJobStr = $"ClassJob::Job(Job::{name})";
                flagName = GetFlagName(name);
            }
            else if (cj.RowId == 25)
            {
                // BlackMage should be Job
                classJobStr = $"ClassJob::Job(Job::{name})";
                flagName = GetFlagName(name);
            }
            else if (cj.RowId == 26)
            {
                // Arcanist should be Class
                classJobStr = $"ClassJob::Class(Class::{name})";
                flagName = GetFlagName(name);
            }
            else if (cj.RowId == 29)
            {
                // Rogue should be Class
                classJobStr = $"ClassJob::Class(Class::{name})";
                flagName = GetFlagName(name);
            }
            else if (cj.RowId == 36)
            {
                // Blue Mage should be Job
                classJobStr = $"ClassJob::Job(Job::{name})";
                flagName = GetFlagName(name);
            }
            else if (cj.RowId == 43)
            {
                // Temporarily disable support for beastmaster.
                continue;
            }            else
            {
                // All other combat jobs (21+ except the special cases above)
                classJobStr = $"ClassJob::Job(Job::{name})";
                flagName = GetFlagName(name);
            }

            sb.Append($"        {classJobStr}.as_str() => JobFlags::{flagName},\n");
        }

        sb.Append("    };\n");
        sb.Append("}\n");

        return sb.ToString();
    }


    private string GenerateRoulettes()
    {
        var sb = DefaultHeader(true);
        sb.Append('\n');
        sb.Append("#[derive(Debug)]\n");
        sb.Append("pub struct RouletteInfo {\n");
        sb.Append("    pub name: LocalisedText,\n");
        sb.Append("    #[expect(unused)]\n");
        sb.Append("    pub pvp: bool,\n");
        sb.Append("}\n\n");

        sb.Append("lazy_static::lazy_static! {\n");
        sb.Append("    pub static ref ROULETTES: HashMap<u32, RouletteInfo> = maplit::hashmap! {\n");

        foreach (var cr in this.Data[Language.English].GetExcelSheet<ContentRoulette>()!)
        {
            if (cr.RowId == 0)
            {
                continue;
            }

            var name = this.GetLocalisedStruct<ContentRoulette>(cr.RowId, row => row.Name, 12);
            if (name == null)
            {
                continue;
            }

            var pvp = cr.IsPvP
                ? "true"
                : "false";

            sb.Append($"        {cr.RowId} => RouletteInfo {{\n");
            sb.Append($"            name: {name},\n");
            sb.Append($"            pvp: {pvp},\n");
            sb.Append("        },\n");
        }

        sb.Append("    };\n");
        sb.Append("}\n");

        return sb.ToString();
    }

    private string GenerateWorlds()
    {
        var DISABLED_WORLDS = new uint[] { 2, 3, 6, 110, 112, 114, 142, 416, 417, 418, 419, 2075, 2076, 2077, 2078, 2080, 2081, 3000, 3001 };

        var sb = DefaultHeader();
        sb.Append("use ffxiv_types::World;\n\n");
        sb.Append("lazy_static::lazy_static! {\n");
        sb.Append("    pub static ref WORLDS: HashMap<u32, World> = maplit::hashmap! {\n");

        foreach (var world in this.Data[Language.English].GetExcelSheet<World>()!)
        {
            if (world.RowId == 0 || !world.IsPublic || world.UserType == 0 || world.DataCenter.RowId == 0 || DISABLED_WORLDS.Contains(world.RowId))
            {
                continue;
            }

            var name = world.Name.ExtractText();
            if (name.Length <= 0)
            {
                continue;
            }

            sb.Append($"        {world.RowId} => World::{name},\n");
        }

        sb.Append("    };\n");
        sb.Append("}\n");

        return sb.ToString();
    }

    private string GenerateTerritoryNames()
    {
        var sb = DefaultHeader(true);
        sb.Append("\nlazy_static::lazy_static! {\n");
        sb.Append("    pub static ref TERRITORY_NAMES: HashMap<u32, LocalisedText> = maplit::hashmap! {\n");

        foreach (var tt in this.Data[Language.English].GetExcelSheet<TerritoryType>()!)
        {
            if (tt.RowId == 0 || tt.PlaceName.RowId == 0)
            {
                continue;
            }

            var name = this.GetLocalisedStruct<TerritoryType>(
                tt.RowId,
                row => row.PlaceName.Value!.Name,
                8
            );
            if (name == null)
            {
                continue;
            }

            sb.Append($"        {tt.RowId} => {name},\n");
        }

        sb.Append("    };\n");
        sb.Append("}\n");

        return sb.ToString();
    }

    private string GenerateAutoTranslate()
    {
        var sb = DefaultHeader(true);
        sb.Append("\nlazy_static::lazy_static! {\n");
        sb.Append("    pub static ref AUTO_TRANSLATE: HashMap<(u32, u32), LocalisedText> = maplit::hashmap! {\n");

        var parser = AutoTranslate.Parser();
        foreach (var row in this.Data[Language.English].GetExcelSheet<Completion>()!)
        {
            // Skip group 61 entries
            if (row.Group == 61)
                continue;

            var lookup = row.LookupTable.ToString().Replace("<num(", "").Replace(")>", ""); // 🙂

            if (lookup is not ("" or "@"))
            {
                string sheetName;
                Maybe<IEnumerable<ISelectorPart>> selector;

                try
                {
                    (sheetName, selector) = parser.ParseOrThrow(lookup);
                }
                catch (Exception ex)
                {
                    Console.WriteLine(lookup);
                    Console.WriteLine(ex.Message);
                    continue;
                }

                var sheets = this.Data
                    .Select(kv => (kv.Key, new ExcelSheet<RawRow>(kv.Value.Excel.GetRawSheet(sheetName))))
                    .ToDictionary();

                var columns = new List<int>();
                var rows = new List<Range>();
                if (selector.HasValue)
                {
                    columns.Clear();
                    rows.Clear();

                    foreach (var part in selector.Value)
                    {
                        switch (part)
                        {
                            case IndexRange range:
                            {
                                var start = (int)range.Start;
                                var end = (int)(range.End + 1);
                                rows.Add(start..end);
                                break;
                            }
                            case SingleRow single:
                            {
                                var idx = (int)single.Row;
                                rows.Add(idx..(idx + 1));
                                break;
                            }
                            case ColumnSpecifier col:
                                columns.Add((int)col.Column);
                                break;
                        }
                    }
                }

                if (columns.Count == 0)
                {
                    columns.Add(0);
                }

                if (rows.Count == 0)
                {
                    rows.Add(..);
                }

                var builder = new StringBuilder();
                foreach (var range in rows)
                {
                    for (var i = (uint)range.Start.Value; i < range.End.Value; i++)
                    {
                        if (!sheets[Language.English].HasRow(i))
                            continue;

                        builder.Clear();
                        builder.Append($"        ({row.Group}, {i}) => LocalisedText {{\n");

                        var lines = 0;
                        foreach (var lang in this.Data.Keys)
                        {
                            var sheet = sheets[lang];

                            var idx = i;
                            if (!sheet.HasRow(idx))
                                continue;

                            var rawRow = sheet.GetRow(idx);
                            foreach (var col in columns)
                            {
                                try
                                {
                                    var seString = rawRow.ReadStringColumn(col);
                                    var text = seString.ExtractText();
                                    if (text.Length > 0)
                                    {
                                        var replace = text.Replace("\\", "\\\\").Replace("\"", "\\\"").Replace(" ", "").Replace("­", "");
                                        builder.Append($"            {Languages[lang]}: \"{replace}\",\n");
                                        lines += 1;
                                        break;
                                    }
                                }
                                catch (ArgumentOutOfRangeException)
                                {
                                    // Column index out of range for this row, skip
                                    continue;
                                }
                            }
                        }

                        builder.Append("        },\n");

                        if (lines != 4)
                        {
                            continue;
                        }

                        sb.Append(builder);
                    }
                }
            }
            else
            {
                var text = this.GetLocalisedStruct<Completion>(row.RowId, row => row.Text, 8);
                if (text != null)
                {
                    sb.Append($"        ({row.Group}, {row.RowId}) => {text},\n");
                }
            }
        }

        sb.Append("    };\n");
        sb.Append("}\n");

        return sb.ToString();
    }

    private string GenerateTreasureMaps()
    {
        var sb = DefaultHeader(true);
        sb.Append("\nlazy_static::lazy_static! {\n");
        sb.Append("    pub static ref TREASURE_MAPS: HashMap<u32, LocalisedText> = maplit::hashmap! {\n");
        sb.Append("        0 => LocalisedText {\n");
        sb.Append("            en: \"All Levels\",\n");
        sb.Append("            ja: \"レベルを指定しない\",\n");
        sb.Append("            de: \"Jede Stufe\",\n");
        sb.Append("            fr: \"Tous niveaux\",\n");
        sb.Append("        },\n");

        var i = 1;
        foreach (var row in this.Data[Language.English].GetExcelSheet<TreasureHuntRank>()!)
        {
            // IS THIS RIGHT?
            if (row.Icon == 0 || row.TreasureHuntTexture != 0)
            {
                continue;
            }

            ReadOnlySeString? GetMapName(TreasureHuntRank thr)
            {
                var name = thr.KeyItemName.Value.Name;
                return string.IsNullOrEmpty(name.ExtractText())
                    ? thr.ItemName.Value.Name
                    : name;
            }

            var name = this.GetLocalisedStruct<TreasureHuntRank>(row.RowId, GetMapName, 8);
            if (!string.IsNullOrEmpty(name))
            {
                sb.Append($"        {i++} => {name},\n");
            }
        }

        sb.Append("    };\n");
        sb.Append("}\n");

        return sb.ToString();
    }
}
