using ZeroElectric.Vinculum;

namespace OpenVegasPro;

public static class Program
{
    public static void Main(string[] args)
    {
        // Set the width and height of the window     
        int screenWidth = 800;
        int screenHeight = 450;

        // Set flags
        Raylib.SetConfigFlags(ConfigFlags.FLAG_WINDOW_RESIZABLE);

        // Initialize the window with the specified width, height, and title
        Raylib.InitWindow(screenWidth, screenHeight, "OpenVegasPro");

        // Set the FPS to 60
        Raylib.SetTargetFPS(60);


        // Loop until the window is closed
        while (!Raylib.WindowShouldClose())
        {
            screenWidth = Raylib.GetScreenWidth();
            screenHeight = Raylib.GetScreenHeight();


            // Begin drawing to the window
            Raylib.BeginDrawing();

            // Clear the background to white
            Raylib.ClearBackground(Raylib.RAYWHITE);
            Raylib.DrawRectangle(0, 0, screenWidth, 32, Raylib.BLUE);
            Raylib.DrawRectangle(0, 32, (int)(screenWidth * 0.4f), (int)(screenHeight * 0.5f), Raylib.RED);
            Raylib.DrawRectangle((int)(screenWidth * 0.4f), 32, (int)(screenWidth * 0.6f), (int)(screenHeight * 0.5f), Raylib.GREEN);
            Raylib.DrawRectangle(0, (int)(screenHeight * 0.5f) + 32, screenWidth, screenHeight - ((int)(screenHeight * 0.5f) + 32), Raylib.PURPLE);



            // End drawing to the window
            Raylib.EndDrawing();
        }

        // Close the window
        Raylib.CloseWindow();
    }
}
