#include "utils.h"

bool Utils::StringContainAny(const std::string& source, std::vector<std::string> matches)
{
	{
		bool contains = false;
		for(std::string &match : matches){
			if(source.find(match) != std::string::npos){
				contains = true;
			}
			if(contains) break;
		}
		return contains;
	}
}
