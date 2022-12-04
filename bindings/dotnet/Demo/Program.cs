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

        PreonColor primaryColor = new PreonColor(0x37, 0x63, 0xF2);
        PreonEngine engine = new();

        engine.Tree = PreonComponent.StartBuilder(engine)
            .FontSize(32.0f)
            .BackgroundColor(PreonColor.White)
            .StartHBox()
                .Expand()
                .StartPanel(primaryColor)
                    .ExpandVertically()
                    .MinSize(new PreonVector<int>(400, 0))
                    .StartVBox()
                        .Padding(new PreonBorder(16, 16, 16, 16))
                        .StartInputField(placeholder: "Placeholder", onChanged: (field, text) =>
                        {
                            Console.WriteLine(text);
                        })
                        .End()
                    .End()
                .End()
                .StartVBox()
                    .Padding(new PreonBorder(16, 16, 16, 16))
                    .Expand()
                    .StartLabel("Label 1").MinSize(new PreonVector<int>(0, 32)).ExpandHorizontally().Id("windowWidth").ForegroundColor(primaryColor).End()
                    .StartLabel("Label 2").MinSize(new PreonVector<int>(0, 32)).ExpandHorizontally().Id("windowHeight").End()
                    .StartLabel("Label 3").MinSize(new PreonVector<int>(0, 32)).ExpandHorizontally().End()
                    .StartLabel("Label 4").MinSize(new PreonVector<int>(0, 32)).ExpandHorizontally().End()
                    .StartLabel("Label 5").MinSize(new PreonVector<int>(0, 32)).ExpandHorizontally().End()
                    .StartButton("Exit").Id("myButton").MinSize(new PreonVector<int>(128, 64)).Padding(new PreonBorder(8, 16, 8, 16)).End()
                .End()
            .End()
        .Build();

        engine.OnResized += newSize => {
            engine.Tree.GetChildById("windowWidth").Text = $"Width: {newSize.X}";
            engine.Tree.GetChildById("windowHeight").Text = $"Height: {newSize.Y}";
        };

        engine.OnPressed("myButton", (button, state) => {
            if (state == Preon.Events.PreonButtonState.Pressed)
                Console.WriteLine("Button Pressed");
        });

        engine.Run();
    }
}