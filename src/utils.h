#pragma once
#include <string>
#include <vector>
#include <opencv2/opencv.hpp>
#include "raylib.h"

class Utils
{
public:
	static bool StringContainAny(const std::string& source, std::vector<std::string> matches);
	static Texture2D GetTextureFromVideoFrame(const cv::Mat &frame);
};