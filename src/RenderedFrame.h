#pragma once
#include "raylib.h"

class RenderedFrame
{
private:
	unsigned long m_frameTick;
	Texture2D m_frameTexture;
	// TODO: add audio support

public:
	RenderedFrame();
	~RenderedFrame();
	void Play();
};