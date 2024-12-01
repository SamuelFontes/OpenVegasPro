// main.cpp : Defines the entry point for the application.
//

#include <iostream>
#include <opencv2/opencv.hpp>

#include "raylib.h"
#define RAYGUI_IMPLEMENTATION
#include "raygui.h"

// Temporarily rename conflicting functions before including windows.h
#define Rectangle WinRectangle
#define CloseWindow WinCloseWindow
#define ShowCursor WinShowCursor
#define DrawText WinDrawText
#include "portable-file-dialogs.h"
// Restore Raylib function names
#undef Rectangle
#undef CloseWindow
#undef ShowCursor
#undef DrawText
#include "video_preview.h"
#include "media_source.h"

typedef struct VideoPlayer
{
	cv::VideoCapture capture;
	cv::Mat frame;
	Rectangle videoRec;
	std::vector<Texture2D> framesToBeRendered;
};
std::atomic<bool> isVideoRunning(false);

Rectangle videoRec = Rectangle{1024, 24, 320, 180};
VideoPreview preview(videoRec);

double fps = 0;
// layout_name: controls initialization
//----------------------------------------------------------------------------------
Rectangle ScrollPanel000ScrollView = { 0, 0, 0, 0 };
Vector2 ScrollPanel000ScrollOffset = { 0, 0 };
Vector2 ScrollPanel000BoundsOffset = { 0, 0 };
//----------------------------------------------------------------------------------


std::vector<MediaSource> mediaSources = {};
int main(int argc, char **argv)
{
	// Load Video
	std::string videoFilePath = "X:/Recordings/2024-06-09_11-00-23.mp4";
	// auto videoFilePath = pfd::open_file("Open", pfd::path::home()).result();

	// Create a video player
	VideoPlayer player = {};

	InitWindow(1366, 768, "Open Vegas Pro");

	bool showMessageBox = false;

	Texture2D texture = {};
	// Directory selection
	// auto dir = pfd::select_folder("Select any directory", pfd::path::home()).result();

	while (!WindowShouldClose())
	{
		if (isVideoRunning)
		{
			// Process video
			player.capture >> player.frame;
			// Check if the frame is empty (end of video)
			if (player.frame.empty())
			{
				isVideoRunning = false;
				fps = 0;
			}
		}
		if (isVideoRunning)
		{
			texture = Utils::GetTextureFromVideoFrame(player.frame);
			preview.SetCurrentFrame(&texture);
		}

		// Draw
		//----------------------------------------------------------------------------------
		BeginDrawing();
		ClearBackground(GetColor(GuiGetStyle(DEFAULT, BACKGROUND_COLOR)));

		DrawRectangle(videoRec.x - 2, videoRec.y - 2, videoRec.width + 4, videoRec.height + 4, BLACK);
		preview.RenderFrame();
		DrawFPS(100, 100);
		int y = 32;
		for (auto &source : mediaSources)
		{
			DrawText(source.GetFileName(), 100, y, 16, BLACK);
			y += 16;
		}

		// Open a video
		if (GuiButton(Rectangle(24, 24, 120, 30), "Open File"))
		{
			auto f = pfd::open_file("Choose files to read", pfd::path::home(),
									{"Video", "*.mp4 *.mkv",
									 "All Files", "*"},
									pfd::opt::none);
			std::cout << "Selected files:";
			for (const std::string& name : f.result())
			{
				MediaSource source(name);
				mediaSources.push_back(source);

				// TODO: move this to the media_clip
				player.capture = cv::VideoCapture(name);
				isVideoRunning = true;
				fps = player.capture.get(cv::CAP_PROP_FPS);
			}

			std::cout << "\n";
		}
		if (GuiButton(Rectangle(200, 24, 120, 30), "Pause"))
		{
			isVideoRunning = !isVideoRunning;
		}

	  GuiScrollPanel(Rectangle( 400, 320, 120 - ScrollPanel000BoundsOffset.x, 72 - ScrollPanel000BoundsOffset.y), NULL, Rectangle(400, 320, 120, 72 ), &ScrollPanel000ScrollOffset, &ScrollPanel000ScrollView);


		EndDrawing();
		if (isVideoRunning)
		{
			UnloadTexture(texture);
			preview.UnloadCurrentFrame();
		}
	}

	CloseWindow();
	return 0;
}
