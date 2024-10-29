// OpenVegasPro.cpp : Defines the entry point for the application.
//

#include "OpenVegasPro.h"
#include <opencv2/opencv.hpp>


int main(int argc, char** argv)
{
	// Load image
	std::string videoFilePath = "X:/Recordings/2024-06-09_11-00-23.mp4";

	// Create a VideoCapture object
	cv::VideoCapture cap(videoFilePath);
	cv::Mat frame;

	InitWindow(1366, 768, "Open Vegas Pro");



	bool showMessageBox = false;


	while (!WindowShouldClose())
	{
		// Draw
		//----------------------------------------------------------------------------------
		BeginDrawing();
		ClearBackground(GetColor(GuiGetStyle(DEFAULT, BACKGROUND_COLOR)));

		// Process video
		cap >> frame;
		// Check if the frame is empty (end of video)
        if (frame.empty()) {
            std::cout << "End of video." << std::endl;
            break;
        }

		// Convert color to be used in opengl
		cv::cvtColor(frame, frame, cv::COLOR_BGR2RGB);

		// cv::imshow("Display Window", img); // use to debug frame
		// cv::waitKey(0);

		// Create a Raylib image from the OpenCV Mat
		Image raylibImage = {
			frame.data, // pixel data
			frame.cols, // width
			frame.rows, // height
			1,          // mipmaps (need to be 1)
			PIXELFORMAT_UNCOMPRESSED_R8G8B8 // format (RGB)
		};

		// Load texture from the Raylib image
		Texture2D texture = LoadTextureFromImage(raylibImage);
		DrawTexture(texture, 0, 0, WHITE);
		DrawFPS(100, 100);

		if (GuiButton(Rectangle(24, 24, 120, 30), "#191#Show Message")) showMessageBox = true;

		if (showMessageBox)
		{
			int result = GuiMessageBox(Rectangle(85, 70, 250, 100),
				"#191#Message Box", "Hi! This is a message!", "Nice;Cool");

			if (result >= 0) showMessageBox = false;
		}

		EndDrawing();
		UnloadTexture(texture);
	}

	CloseWindow();
	return 0;
}
