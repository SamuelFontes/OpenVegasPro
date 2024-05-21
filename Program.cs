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
			// Begin drawing to the window
			Raylib.BeginDrawing();

			// Clear the background to white
			Raylib.ClearBackground(Raylib.RAYWHITE);

			// Draw the text "Hello World" in maroon color at position (190, 200)
			Raylib.DrawText("Hello Raylib in CSharp!", 190, 200, 20, Raylib.MAROON);

			// End drawing to the window
			Raylib.EndDrawing();
		}

		// Close the window
		Raylib.CloseWindow();
	}
}
