import asyncio

import gi

gi.require_version("IgnisNotificationsGLib", "0.1")
from gi.events import GLibEventLoopPolicy  # noqa: E402
from gi.repository import GLib, IgnisNotificationsGLib  # noqa: E402 # type: ignore


def setup_asyncio():
    policy = GLibEventLoopPolicy()
    asyncio.set_event_loop_policy(policy)


def test_notification():
    setup_asyncio()
    mainloop = GLib.MainLoop()

    test_exception = None

    async def async_run():
        a = IgnisNotificationsGLib.Service.new()
        await a.run_async()

        subprocess = await asyncio.create_subprocess_exec(
            "notify-send",
            "summary",
            "body",
            stdout=asyncio.subprocess.PIPE,
            stderr=asyncio.subprocess.PIPE,
        )

        stdout, stderr = await subprocess.communicate()
        print(f"notify-send exited with\nstdout: {stdout}\nstderr: {stderr}")

        assert subprocess.returncode == 0

    def on_task_done(task):
        nonlocal test_exception
        try:
            task.result()
        except Exception as e:
            test_exception = e

        mainloop.quit()

    def main():
        task = asyncio.create_task(async_run())
        task.add_done_callback(on_task_done)

    GLib.idle_add(main)
    mainloop.run()

    if test_exception:
        raise test_exception


if __name__ == "__main__":
    test_notification()
