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
                .FontSize(32.0f)
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
                        .StartLabel("Label 1").MinSize(new PreonVector<int>(0, 32)).ExpandHorizontally().Id("windowWidth").ForegroundColor(new PreonColor(0.6f, 0.6f, 1.0f)).End()
                        .StartLabel("Label 2").MinSize(new PreonVector<int>(0, 32)).ExpandHorizontally().Id("windowHeight").End()
                        .StartLabel("Label 3").MinSize(new PreonVector<int>(0, 32)).ExpandHorizontally().End()
                        .StartLabel("Label 4").MinSize(new PreonVector<int>(0, 32)).ExpandHorizontally().End()
                        .StartLabel("Label 5").MinSize(new PreonVector<int>(0, 32)).ExpandHorizontally().End()
                    .End()
                .End()
            .Build()
        );

        engine.Run((tree, @event, userEvents) => {
            switch (@event)
            {
                case PreonEvent.WindowResized resizeEvent:
                    tree.GetChildById("windowWidth").Text = $"Width: {resizeEvent.NewSize.X}";
                    tree.GetChildById("windowHeight").Text = $"Height: {resizeEvent.NewSize.Y}";
                    break;
                default: break;
            }
        });
    }
}