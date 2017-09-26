from telegram import *
from telegram.ext import *

isOn = False
currentUserName = ""
currentUserId = ""

keyboard_turn_on = [[InlineKeyboardButton("Ligar moedor", callback_data='turn_on')]]
keyboard_turn_off = [[InlineKeyboardButton("Desligar moedor", callback_data='turn_off')]]


def is_in_use():
    return isOn


def set_user(user_name, user_id):
    global isOn, currentUserName, currentUserId
    isOn = True
    currentUserId = user_id
    currentUserName = user_name
    return


def clear_user():
    global isOn, currentUserName, currentUserId
    isOn = False
    currentUserName = ""
    currentUserId = ""
    return


def start(bot, update):
    if isOn and update.effective_user.id != currentUserId:
        update.message.reply_text(currentUserName + ' já está usando a máquina. Por favor, aguarde.')
        return

    if isOn and update.effective_user.id == currentUserId:
        reply_markup = InlineKeyboardMarkup(keyboard_turn_off)
        update.message.reply_text('Você já está usando o moedor.', reply_markup=reply_markup)
        return
        
    reply_markup = InlineKeyboardMarkup(keyboard_turn_on)
    update.message.reply_text('O que você deseja fazer?', reply_markup=reply_markup)


def build_menu(buttons,
               n_cols,
               header_buttons=None,
               footer_buttons=None):
    menu = [buttons[i:i + n_cols] for i in range(0, len(buttons), n_cols)]
    if header_buttons:
        menu.insert(0, header_buttons)
    if footer_buttons:
        menu.append(footer_buttons)
    return menu


def button(bot, update):
    query = update.callback_query
    if not is_in_use():
        if query.data == 'turn_on':
            set_user(update.effective_user.first_name, update.effective_user.id)
            reply_markup = InlineKeyboardMarkup(keyboard_turn_off)
            bot.edit_message_text(text="Todo uso do moedor será creditado à você durante os próximos 5 minutos ou até você clicar no botão abaixo",
                                  chat_id=query.message.chat_id,
                                  message_id=query.message.message_id,
                                  reply_markup=reply_markup)
            return
    if query.data == 'turn_off' and update.effective_user.id == currentUserId:
        clear_user()
        reply_markup = InlineKeyboardMarkup(keyboard_turn_on)
        bot.edit_message_text(
            text="O moedor foi desligado. O uso do moedor não estava mais atrelado a sua conta.",
            chat_id=query.message.chat_id,
            message_id=query.message.message_id,
            reply_markup=reply_markup)
        return
    if query.data == 'turn_off' and update.effective_user.id != currentUserId:
        reply_markup = InlineKeyboardMarkup(keyboard_turn_on)
        bot.edit_message_text(
            text="O uso do moedor não sera mais creditado a sua conta.",
            chat_id=query.message.chat_id,
            message_id=query.message.message_id,
            reply_markup=reply_markup)
        return




updater = Updater('477595004:AAGjB-rB5Mwkc75D5Q9E_2RqUBi2ioinCSk')

updater.dispatcher.add_handler(CommandHandler('start', start))
updater.dispatcher.add_handler(CallbackQueryHandler(button))
updater.dispatcher.add_handler(CommandHandler('help', help))

def help(bot, update):
    update.message.reply_text("Digite /start para usar este bot.")

# Start the Bot
updater.start_polling()

# Run the bot until the user presses Ctrl-C or the process receives SIGINT,
# SIGTERM or SIGABRT
updater.idle()
