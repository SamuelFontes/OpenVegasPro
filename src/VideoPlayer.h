#pragma once
#include "raylib.h"
#include "VideoPreview.h"
#include "RenderedFrame.h"
#include <vector>
#include <iostream>

class VideoPlayer {
private:
	bool m_isVideoPlaying;
	unsigned long m_currentTick;
	unsigned long m_tickAmount;
	Rectangle m_playerRenderingRectangle;
	VideoPreview m_videoPreview = VideoPreview(Rectangle{1024, 24, 320, 180});
	std::vector<RenderedFrame> m_framesBuffered;
	RenderedFrame* m_currentFrame;

public:
	VideoPlayer();
	~VideoPlayer();
};