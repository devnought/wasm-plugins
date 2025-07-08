using HostExtensionWorld;

public class HostExtensionWorldImpl : IHostExtensionWorld
{
    public static List<string> Run(string msg)
    {
        HostExtensionWorld.exports.HostExtensionWorld.Print("Message from C#");
        return msg.Split(' ').ToList();
    }
}