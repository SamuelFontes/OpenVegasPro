#include "user_interface.h"

UserInterface::UserInterface()
{
	InitWindow(1366, 768, "Open Vegas Pro");
}

UserInterface::~UserInterface()
{
	CloseWindow();
}

void UserInterface::UpdateAndDraw()
{
}
