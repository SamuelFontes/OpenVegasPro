#pragma once
#include <string>
#include "enums.h"
#include "raylib.h"
#include <opencv2/opencv.hpp>
#include "media_source.h"

class MediaClip
{
private:
	MediaSource* m_mediaSource;
	MediaType m_mediaType;
	cv::VideoCapture m_videoCapture;
	cv::Mat m_currentFrame;
	unsigned long m_mediaStartTick;
	unsigned long m_mediaEndTick;
	unsigned int m_layer;
	// TODO: handle audio
	// TODO: add effects and shit


public:
	MediaClip(MediaSource* source, unsigned long start, unsigned int layer = 0);
	void RenderTexture(int keyFrame, Texture2D& texture);
	void MixAudio(int keyFrame); // TODO: see how to get the audio for the current keyFrame
};