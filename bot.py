from telegram import *
from telegram.ext import *
import time


isOn = False
currentUserName = ""
currentUserId = ""
currentUserChatId = ""
time_left_to_off = 0
turn_on_msg_id = ""
keyboard_turn_on = [[InlineKeyboardButton("Ligar moedor", callback_data='turn_on')]]


def keyboard_turn_off():
    return [[InlineKeyboardButton("Desligar o moedor " + str(int(time_left_to_off)), callback_data='turn_off')]]


def is_in_use():
    return isOn


def set_user(user_name, user_id, chat_id):
    global isOn, currentUserName, currentUserId, time_left_to_off, currentUserChatId
    isOn = True
    currentUserId = user_id
    currentUserName = user_name
    currentUserChatId = chat_id
    time_left_to_off = 300
    return


def clear_user():
    global isOn, currentUserName, currentUserId
    isOn = False
    currentUserName = ""
    currentUserId = ""
    return


def start(bot, update):
    if isOn and update.effective_user.id != currentUserId:
        update.message.reply_text(currentUserName + ' já está usando a máquina. Por favor, aguarde ' + str(int(time_left_to_off)) + 'segundos.')
        return

    if isOn and update.effective_user.id == currentUserId:
        reply_markup = InlineKeyboardMarkup(keyboard_turn_off())
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
            global turn_on_msg_id, context
            turn_on_msg_id = query.message.message_id
            set_user(update.effective_user.first_name, update.effective_user.id, update.effective_chat.id)
            reply_markup = InlineKeyboardMarkup(keyboard_turn_off())
            context = query
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


while 1:
    if isOn:
        if time_left_to_off > 0:
            last_time = time.time()
            time.sleep(1)
            time_now_millis = time.time()
            if time_left_to_off-(time_now_millis-last_time) > 0:
                time_left_to_off = time_left_to_off - (time_now_millis - last_time)
                reply_markup = InlineKeyboardMarkup(keyboard_turn_off())
                updater.bot.edit_message_reply_markup(
                    text="Todo uso do moedor será creditado à você durante os próximos 5 minutos ou até você clicar no botão abaixo",
                    chat_id=currentUserChatId,
                    message_id=turn_on_msg_id,
                    reply_markup=reply_markup)
            else:
                time_left_to_off = 0
                clear_user()
                updater.bot.send_message(
                    text="O moedor foi desligado. O uso do moedor não esta mais atrelado a sua conta.",
                    chat_id=currentUserChatId)



