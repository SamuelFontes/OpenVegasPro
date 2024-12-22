#pragma once
#include "raylib.h"
#include "../media_source.h"
#include <vector>

class UserInterface
{
private:
	std::vector<MediaSource> m_mediaSources = {};
	
public:
	UserInterface();
	~UserInterface();

	void UpdateAndDraw();
};