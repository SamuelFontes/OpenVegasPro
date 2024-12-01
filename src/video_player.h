#pragma once
#include "raylib.h"
#include "video_preview.h"
#include "rendered_frame.h"
#include <vector>

class VideoPlayer {
private:
	bool m_isVideoPlaying;
	unsigned long m_currentTick;
	unsigned long m_tickAmount;
	Rectangle m_playerRenderingRectangle;
	VideoPreview m_videoPreview;
	std::vector<RenderedFrame> m_framesBuffered;
	RenderedFrame& m_currentFrame;

public:
	VideoPlayer();
	~VideoPlayer();
};