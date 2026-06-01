# TODO: make it an actual test (pytest)
import gi

gi.require_version("IgnisNotificationsGLib", "0.1")
from gi.repository import IgnisNotificationsGLib, GLib


def main():
    a = IgnisNotificationsGLib.Service.new()
    a.run_async(None, lambda x, res: a.run_finish(res))


loop = GLib.MainLoop()
GLib.idle_add(main)
loop.run()
