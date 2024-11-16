#pragma once
#include <string>
#include <regex>
#include "enums.h"
#include "utils.h"

class MediaSource{
private:
	std::string m_mediaPath;
    std::string m_fileName;
	MediaType m_mediaType;

public:
    MediaSource(const std::string& filePath);
    char* GetFileName();
};