#include "media_source.h"

MediaSource::MediaSource(const std::string& filePath)
{
    // Get media type
    if(StringContainAny(filePath, {".mp4",".mkv"})) // TODO: add all supported formats here, or move it elsewhere
    {
        m_mediaType = MediaType::Video;
        // TODO: add audio track
    }
    else if(StringContainAny(filePath, {".mp3",".obb"}))
        m_mediaType = MediaType::Audio;
    else if(StringContainAny(filePath, {".png",".jpg",".jpeg"}))
        m_mediaType = MediaType::Image;
    
    // get fileName
    std::regex getFileNamePattern("[^/]*$"); 
    std::smatch result;
    if(std::regex_search(filePath, result, getFileNamePattern)){
        m_fileName = result[0];
    }
    else m_fileName = filePath; // just in case my regex doesn't work
}

char* MediaSource::GetFileName()
{
    return m_fileName.data();
}
