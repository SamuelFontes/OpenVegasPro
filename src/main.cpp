// OpenVegasPro.cpp : Defines the entry point for the application.
//

#include "OpenVegasPro.h"
#include <opencv2/opencv.hpp>


int main(int argc, char** argv)
{
	// Load image
    //std::string imagePath = "X:/Recordings/2024-06-09_11-00-23.mp4"; 
	cv::Mat img = cv::imread("C:/Users/sfontes/Pictures/Saved Pictures/discord_pic.png");


	// Convert color to be used in opengl
	cv::cvtColor(img, img, cv::COLOR_BGR2RGB);

	// cv::imshow("Display Window", img); // use to debug frame
	// cv::waitKey(0);

	// Create a Raylib image from the OpenCV Mat
	Image raylibImage = {
		img.data, // pixel data
		img.cols, // width
		img.rows, // height
		1,          // mipmaps (need to be 1)
		PIXELFORMAT_UNCOMPRESSED_R8G8B8 // format (RGB)
	};

	InitWindow(1366, 768, "Open Vegas Pro");

	// Load texture from the Raylib image
	Texture2D texture = LoadTextureFromImage(raylibImage);


	bool showMessageBox = false;

	while (!WindowShouldClose())
	{
		// Draw
		//----------------------------------------------------------------------------------
		BeginDrawing();
		ClearBackground(GetColor(GuiGetStyle(DEFAULT, BACKGROUND_COLOR)));

		DrawTexture(texture, 0, 0, WHITE);

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
