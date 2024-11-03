#pragma once
#include <string>
#include "enums.h"
#include "raylib.h"
#include <opencv2/opencv.hpp>

class MediaClip
{
private:
	std::string m_mediaPath;
	MediaType m_mediaType;
	int m_mediaStartKeyFrame;
	int m_mediaEndKeyFrame;


public:
	MediaClip(std::string path, MediaType type, int start, int end);
	void RenderTexture(int keyFrame, Texture2D& texture);
	void MixAudio(int keyFrame); // TODO: see how to get the audio for the current keyFrame
};