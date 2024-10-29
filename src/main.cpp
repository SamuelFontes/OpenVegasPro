// OpenVegasPro.cpp : Defines the entry point for the application.
//

#include "OpenVegasPro.h"


using namespace std;

int main(int argc, char** argv)
{
	InitWindow(1366, 768, "Open Vegas Pro");

	bool showMessageBox = false;

	while (!WindowShouldClose())
	{
		// Draw
		//----------------------------------------------------------------------------------
		BeginDrawing();
		ClearBackground(GetColor(GuiGetStyle(DEFAULT, BACKGROUND_COLOR)));

		if (GuiButton(Rectangle(24, 24, 120, 30), "#191#Show Message")) showMessageBox = true;

		if (showMessageBox)
		{
			int result = GuiMessageBox(Rectangle(85, 70, 250, 100),
				"#191#Message Box", "Hi! This is a message!", "Nice;Cool");

			if (result >= 0) showMessageBox = false;
		}

		EndDrawing();
	}

	CloseWindow();
	return 0;
}
