import curses
import sys


def main(argv=sys.argv):
    active = True
    try:
        while active:
            screen = curses.initscr()
            screen.border(0)
            screen.addstr(2, 2, '                            \u2022\u2022')
            screen.addstr(3, 2, '                            \u2022\u2022')
            screen.addstr(4, 2, '   \u2022\u2022\u2022\u2022\u2022\u2022    \
\u2022\u2022\u2022\u2022\u2022\u2022    \
\u2022\u2022\u2022\u2022\u2022\u2022\u2022')
            screen.addstr(5, 2, '  \u2022\u2022    \u2022\u2022        \
\u2022\u2022  \u2022\u2022    \u2022\u2022')
            screen.addstr(6, 2, '  \u2022\u2022    \u2022   \
\u2022\u2022\u2022\u2022\u2022\u2022\u2022\u2022  \u2022\u2022    \u2022\u2022 \
 -  Once a day')
            screen.addstr(7, 2, '  \u2022\u2022    \u2022\u2022  \u2022\u2022  \
  \u2022\u2022  \u2022\u2022    \u2022\u2022')
            screen.addstr(8, 2, '   \u2022\u2022\u2022\u2022\u2022\u2022    \
\u2022\u2022\u2022\u2022\u2022\u2022\u2022   \
\u2022\u2022\u2022\u2022\u2022\u2022\u2022')
            screen.addstr(11, 4, '1 - Mailsettings')
            screen.addstr(12, 4, '2 - Time Settings')
            screen.addstr(13, 4, '3 - Notifications')
            screen.addstr(14, 4, '4 - Exit')
            screen.refresh()

            choice = screen.getch()

            if choice == ord('4'):
                active = False
                curses.endwin()
            elif choice == ord('1'):
                # TODO: write function
                utils.mail()
                curses.endwin()
            elif choice == ord('2'):
                # TODO: write function
                utils.time()
                curses.endwin()
            elif choice == ord('3'):
                utils.notification()
                curses.endwin()
    except Exception as e:
        curses.endwin()


def get_param(prompt_string):
    screen.clear()
    screen.border(0)
    screen.addstr(2, 2, prompt_string)
    screen.refresh()
    input = screen.getstr(10, 10, 60)
    return input
if __name__ == '__main__':
    main()
