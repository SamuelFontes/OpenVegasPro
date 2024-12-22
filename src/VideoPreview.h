#pragma once
#include "raylib.h"

class VideoPreview {
public:
	VideoPreview(Rectangle rec); 
	
	void RenderFrame();
	void SetCurrentFrame(Texture2D* frame);
	void UnloadCurrentFrame();

private:
	bool m_hasFrameToRender;
	Texture2D* m_currentFrame;
	Rectangle m_renderingRectangle;

};
