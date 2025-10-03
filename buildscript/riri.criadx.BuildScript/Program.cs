namespace riri.criadx.BuildScript;

public class SkipGlobals : Argument
{
    public override void HandleParams(string[] args)
    {
        Enabled = args[0].ToLower() switch
        {
            "true" => true,
            "false" => false,
            _ => throw new Exception($"Expected a boolean value, got {args[0]} instead")
        };
    }

    public override int GetParamCount() => 1;
}

public class Timings : Argument
{
    public override void HandleParams(string[] args)
    {
        Enabled = args[0].ToLower() switch
        {
            "true" => true,
            "false" => false,
            _ => throw new Exception($"Expected a boolean value, got {args[0]} instead")
        };
    }

    public override int GetParamCount() => 1;
}

public class Publish : Argument
{
    public override void HandleParams(string[] args)
    {
        Enabled = args[0].ToLower() switch
        {
            "true" => true,
            "false" => false,
            _ => throw new Exception($"Expected a boolean value, got {args[0]} instead")
        };
    }

    public override int GetParamCount() => 1;
}

public class ArgumentList : ArgumentListBase
{
    public ArgumentList(string[] args) : base(args) { }

    protected override Dictionary<string, Argument> SetArguments()
    {
        return new()
        {
            { "Debug", new Debug() },
            { "SkipGlobals", new SkipGlobals() },
            { "Timings", new Timings() },
            { "Publish", new Publish() }
        };
    }
}

public class ProjectManager : ProjectManagerBase
{
    public override List<KeyValuePair<string, CodePackage>> GetProjects(ArgumentListBase arg, string RootPath)
    {
        return new List<KeyValuePair<string, CodePackage>>()
        {
            Register(new CSharpProject(arg, Path.Combine(RootPath, "riri.criadx"))),
            Register(new RustCrate(arg, Path.Combine(RootPath, "cri-adx-reloaded"))),
        };
    }
    public ProjectManager(ArgumentList arg, string RootPath) : base(arg, RootPath) { }
}

public class Executor : ExecutorBase<ArgumentList, ProjectManager>
{
    public override string BuildType
    {
        get => "CLIENT";
    }

    public Executor(string[] args) : base(args) { }

    public override void Execute()
    {
        if (ArgList["Publish"].Enabled)
        {
            PublishState.Cleanup();
            PublishState.GetTools();
        }
        PrintInformation();
        // Create riri_hook folder if it doesn't already exist
        Directory.CreateDirectory(Path.Combine(ProjectManager["cri-adx-reloaded"].RootPath, "riri_hook"));
        // Build Cri ADX Bindings (Rust portion)
        ProjectManager["cri-adx-reloaded"].Build();
        // Build Cri ADX Bindings (C# portion)
        if (ArgList["Publish"].Enabled)
        {
            ((CSharpProject)ProjectManager["riri.criadx"]).PublishBuildDirectory = PublishState.PublishBuildDirectory;
            ((CSharpProject)ProjectManager["riri.criadx"]).TempDirectory = PublishState.TempDirectoryBuild;
        }
        ProjectManager["riri.criadx"].Build();
        if (ArgList["Publish"].Enabled)
        {
            PublishState.CreateArtifacts();
        }
        else
        {
            // Copy output files from target folder into Reloaded mod
            var reloadedDirectory = Path.Combine(Environment.GetEnvironmentVariable("RELOADEDIIMODS")!, "riri.criadx");
            ((RustCrate)ProjectManager["cri-adx-reloaded"]).CopyOutputArtifacts(ArgList["Debug"].Enabled, RootPath, reloadedDirectory);
        }
        PrintCompleted();
    }
}

internal class Program
{
    static void Main(string[] args)
    {
        if (Environment.GetEnvironmentVariable("RELOADEDIIMODS") == null)
            throw new Exception("The environment variable RELOADEDIIMODS is not defined!");
        var exec = new Executor(args);
        exec.Execute();
    }
}
