#include "media_clip.h"

MediaClip::MediaClip(MediaSource* source, unsigned long start, unsigned int layer = 0)
{
	m_mediaSource = source;
	m_mediaStartTick = start;
	m_mediaEndTick = start; // TODO: get media lenght and convert to tickerate
	m_layer = layer;
}

void MediaClip::RenderTexture(int keyFrame, Texture2D& texture)
{
	// Convert color to be used in opengl
	//cv::cvtColor(player.frame, player.frame, cv::COLOR_BGR2RGB);

	//// cv::imshow("Display Window", img); // use to debug frame
	//// cv::waitKey(0);

	//// Create a Raylib image from the OpenCV Mat
	//Image raylibImage = {
	//  player.frame.data, // pixel data
	//  player.frame.cols, // width
	//  player.frame.rows, // height
	//  1,          // mipmaps (need to be 1)
	//  PIXELFORMAT_UNCOMPRESSED_R8G8B8 // format (RGB)
	//};

	//// Load texture from the Raylib image
	//texture = LoadTextureFromImage(raylibImage); // TODO: this needs to be buffered, loading a texture every frame is bad
}

void MediaClip::MixAudio(int keyFrame)
{
	if(m_mediaType != MediaType::Audio) return;
}

