# CoffeezeraBot
A Telegram bot which manages a coffee grinder.

**Video of the project working:** https://photos.app.goo.gl/vcP8ovgUfEwVDuQQ9

The user activates it using the Telegram Bot Coffeezera. 

The bot allows the user to turn on the grinding (just turn on, not start grinding), turn off, and view its available credits.

The grinder is turned on and off using a relay. 

The bot only deduces credits when the user is actually grinding. It knows when it's grinding by measuring the current flowing to the grinder.

Circuit to measure the current (the current is AC, so its a bit tricky): 

https://easyeda.com/tiberiusferreira/Coffeezera-06a3484d75f944d485998dde287004a5

Outputs either 0V when it is not grinding or 3.15V when it is. 


