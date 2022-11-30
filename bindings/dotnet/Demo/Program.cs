using Preon;
using Preon.Events;
using Preon.Types;

internal class Program
{
    [STAThread]
    private static void Main(string[] args)
    {
        PreonEngine.Init();

        PreonEngine engine = new();

        engine.SetTree(
            PreonComponent.StartBuilder()
                .FontSize(64.0f)
                .BackgroundColor(new PreonColor(0.0f, 0.0f, 0.0f))
                .ForegroundColor(new PreonColor(0.8f, 0.8f, 0.8f))
                .StartHBox()
                    .Expand()
                    .StartPanel(new PreonColor(0.6f, 0.6f, 1.0f))
                        .ExpandVertically()
                        .MinSize(new PreonVector<int>(400, 0))
                        .StartVBox()
                        .End()
                    .End()
                    .StartVBox()
                        .Padding(new PreonBorder(16, 16, 16, 16))
                        .Expand()
                        // .BackgroundColor(new PreonColor(1.0f, 1.0f, 1.0f))
                        .StartLabel("Label 1").Id("windowWidth").ForegroundColor(new PreonColor(0.6f, 0.6f, 1.0f)).MinSize(new PreonVector<int>(200, 64)).End()
                        .StartLabel("Label 2").Id("windowHeight").MinSize(new PreonVector<int>(200, 64)).End()
                        .StartLabel("Label 3").MinSize(new PreonVector<int>(200, 64)).End()
                        .StartLabel("Label 4").MinSize(new PreonVector<int>(200, 64)).End()
                        .StartLabel("Label 5").MinSize(new PreonVector<int>(200, 64)).End()
                    .End()
                .End()
            .Build()
        );

        engine.Run((tree, @event, userEvents) => {
            tree.Test();
            Console.WriteLine($"{tree} {@event} {userEvents}");
            switch (@event)
            {
                case PreonEvent.WindowResized resizeEvent:
                    Console.WriteLine($"Resized: {resizeEvent.NewSize.X} {resizeEvent.NewSize.Y}");
                    tree.Test();
                    tree.Text = "Hi";
                    // Console.WriteLine(tree.GetChildById("windowWidth"));//.Text;// = $"Width: {resizeEvent.NewSize.X}";
                    // tree.GetChildById("windowHeight").Text = $"Height: {resizeEvent.NewSize.Y}";
                    break;
                default: break;
            }
        });
    }
}