// main.cpp : Defines the entry point for the application.
//

#include <iostream>
#include <opencv2/opencv.hpp>


#include "raylib.h"
// TODO: learn why this is needed
#define RAYGUI_IMPLEMENTATION
#include "raygui.h"

// Temporarily rename conflicting functions before including windows.h
#define Rectangle WinRectangle
#define CloseWindow WinCloseWindow
#define ShowCursor WinShowCursor
#include "external/portable-file-dialogs.h" // TODO: fix this shit on windows, the Rectangle thing is confliting with the Raylib one
// Restore Raylib function names
#undef Rectangle
#undef CloseWindow
#undef ShowCursor


int main(int argc, char** argv)
{
	// Load Video
	std::string videoFilePath = "X:/Recordings/2024-06-09_11-00-23.mp4";
	//auto videoFilePath = pfd::open_file("Open", pfd::path::home()).result();

	// Create a VideoCapture object
	cv::VideoCapture cap;
	cv::Mat frame;

	InitWindow(1366, 768, "Open Vegas Pro");

	bool showMessageBox = false;

	SetTargetFPS(60); // TODO: remove this after the frame loading is running in another core

	// Directory selection
	//auto dir = pfd::select_folder("Select any directory", pfd::path::home()).result();
	bool isVideoRunning = false;

	while (!WindowShouldClose())
	{
		Texture2D texture = {};
		if (isVideoRunning)
		{
			// Process video
			cap >> frame;
			// Check if the frame is empty (end of video)
			if (frame.empty()) {
				isVideoRunning = false;
			}
		}
		if (isVideoRunning)
		{

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
			texture = LoadTextureFromImage(raylibImage); // TODO: this needs to be buffered, loading a texture every frame is bad
		}


		// Draw
		//----------------------------------------------------------------------------------
		BeginDrawing();
		ClearBackground(GetColor(GuiGetStyle(DEFAULT, BACKGROUND_COLOR)));

		if (isVideoRunning) DrawTexture(texture, 0, 0, WHITE);
		DrawFPS(100, 100);

		if (GuiButton(Rectangle(24, 24, 120, 30), "#191#Show Message")) showMessageBox = true;
		if (GuiButton(Rectangle(300, 24, 120, 30), "Open File")) 
		{
			auto f = pfd::open_file("Choose files to read", pfd::path::home(),
                            { "Video", "*.mp4 *.mkv",
                              "All Files", "*" },
                            pfd::opt::none);
			std::cout << "Selected files:";
			for (auto const& name : f.result()) {
				cap = cv::VideoCapture(name);
				isVideoRunning = true;
			}

			std::cout << "\n";

		}

		if (showMessageBox)
		{
			int result = GuiMessageBox(Rectangle(85, 70, 250, 100),
				"#191#Message Box", "Hi! This is a message!", "Nice;Cool");

			if (result >= 0) showMessageBox = false;
		}

		EndDrawing();
		if (isVideoRunning) UnloadTexture(texture);
	}

	CloseWindow();
	return 0;
}
