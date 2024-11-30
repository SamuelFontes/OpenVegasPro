#include "utils.h"

bool Utils::StringContainAny(const std::string& source, std::vector<std::string> matches)
{
	{
		bool contains = false;
		for(std::string& match : matches)
		{
			if(source.find(match) != std::string::npos)
			{
				contains = true;
			}
			if(contains) break;
		}
		return contains;
	}
}

Texture2D Utils::GetTextureFromVideoFrame(const cv::Mat& frame)
{
	// Convert color to be used in opengl
	cv::cvtColor(frame, frame, cv::COLOR_BGR2RGB);

	// Create a Raylib image from the OpenCV Mat
	Image raylibImage = {
		frame.data,				// pixel data
		frame.cols,				// width
		frame.rows,				// height
		1,								// mipmaps (need to be 1)
		PIXELFORMAT_UNCOMPRESSED_R8G8B8 // format (RGB)
	};

	// Load texture from the Raylib image
	return LoadTextureFromImage(raylibImage); 
}
