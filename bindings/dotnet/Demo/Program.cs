using Preon;
using Preon.Events;
using Preon.Types;

namespace Demo;

internal class Program
{
    [STAThread]
    private static void Main(string[] args)
    {
        PreonEngine.Init();

        PreonColor sidebarColor = PreonColor.Hex("#444")!.Value;
        PreonColor primaryColor = PreonColor.Hex("#da0037")!.Value;
        PreonColor secondaryColor = PreonColor.Hex("#333")!.Value;
        PreonColor backgroundColor = PreonColor.Hex("#333")!.Value;
        PreonColor foregroundColor = PreonColor.Hex("#d3d3d3")!.Value;
        PreonEngine engine = new();

        engine.Tree = PreonComponent.StartBuilder(engine)
            .FontSize(32.0f)
            .BackgroundColor(backgroundColor)
            .ForegroundColor(foregroundColor)
            .StartHBox()
                .Expand()
                .StartPanel(sidebarColor)
                    .ExpandVertically()
                    .MinSize(new PreonVector<int>(400, 0))
                    .StartVBox()
                        .Padding(new PreonBorder(16))
                        .StartInputField(
                            placeholder: "Search...",
                            placeholderColor: PreonColor.Light(0.5f),
                            onChanged: (field, text) =>
                            {
                                Console.WriteLine(text);
                            })
                            .BackgroundColor(secondaryColor)    
                        .End()
                    .End()
                .End()
                .StartVBox()
                    .Padding(new PreonBorder(16))
                    .Expand()
                    .StartLabel("Label 1").MinSize(new PreonVector<int>(0, 32)).ExpandHorizontally().Id("windowWidth").ForegroundColor(primaryColor).End()
                    .StartLabel("Label 2").MinSize(new PreonVector<int>(0, 32)).ExpandHorizontally().Id("windowHeight").End()
                    .StartLabel("Label 3").MinSize(new PreonVector<int>(0, 32)).ExpandHorizontally().End()
                    .StartLabel("Label 4").MinSize(new PreonVector<int>(0, 32)).ExpandHorizontally().End()
                    .StartLabel("Label 5").MinSize(new PreonVector<int>(0, 32)).ExpandHorizontally().End()
                    .StartButton("Exit")
                        .Id("myButton")
                        .CornerRadius(new PreonCorners() { topLeft = 10.0f, topRight = 15.0f, bottomLeft = 20.0f, bottomRight = 25.0f})
                        .BackgroundColor(primaryColor)
                        .MinSize(new PreonVector<int>(128, 64))
                        .Padding(new PreonBorder(8, 16))
                    .End()
                .End()
            .End()
        .Build();

        engine.OnResized += newSize => {
            engine.Tree.GetChildById("windowWidth").Text = $"Width: {newSize.x}";
            engine.Tree.GetChildById("windowHeight").Text = $"Height: {newSize.y}";
        };

        engine.OnPressed("myButton", (button, state) => {
            if (state == Preon.Events.PreonButtonState.Pressed)
                Console.WriteLine("Button Pressed");
        });

        engine.Run();
    }
}