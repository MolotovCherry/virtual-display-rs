from pyvdd import *

#
# Before you begin, please note the following:
#
# 1. All Monitors must have a unique Id
# 2. All Modes under a Monitor must have a unique width/height
# 3. All refresh rates under a Mode must be unique
#
# Every class, attribute, and function is annotated with a __doc__, which also shows the type's
# type signature.
#
# Final note: It is possible to have stale data in memory, and this can cause duplicate Ids.
#             However, if it is sent to the driver, the driver will simply ignore the duplicates.
#             When notify() is done, it DOES NOT check the latest data! You must reconcile differences via
#             either get_state(), or set up a receiver() to be notified of new changes
#

# make the client
client = DriverClient()
# you can see what's in it
print(client)
# DriverClient { monitors: [Monitor { id: 0, name: None, enabled: true, modes: [Mode { width: 1920, height: 1080, refresh_rates: [90, 120] }] }] }

# monitors are stored at
print(client.monitors)
# [Monitor { id: 0, name: None, enabled: true, modes: [Mode { width: 1920, height: 1080, refresh_rates: [90, 120] }] }]

#
# Monitor functionality
#

# to get a monitor, just index
client.monitors[0]
# set id
client.monitors[0].id = 0
# set name
client.monitors[0].name = "MyName"
# you can unset the name
client.monitors[0].name = None
# enable or disable monitor
client.monitors[0].enabled = False

# delete a monitor we don't want
del client.monitors[0]

# create new monitor (set it up as you want)
new_mon = Monitor()
client.monitors[0] = new_mon

# add a new monitor to list
client.monitors += Monitor()
# or add multiple
client.monitors += [Monitor(), Monitor()]

# you can iterate over them
for mon in client.monitors:
    print(mon)
    # Monitor { id: 0, name: None, enabled: true, modes: [Mode { width: 1920, height: 1080, refresh_rates: [90, 120] }] }
    print(mon.modes)
    # [Mode { width: 1920, height: 1080, refresh_rates: [90, 120] }]

#
# Modes
#

# access a mode by index
print(client.monitors[0].modes[0])
# Mode { width: 1920, height: 1080, refresh_rates: [90, 120] }

# set width
client.monitors[0].modes[0].width = 1000
# set height
client.monitors[0].modes[0].height = 1000
# check out refresh rates
print(client.monitors[0].modes[0].refresh_rates)
# [90, 120]

# add a new mode
new_mode = Mode()
# set up properties like normal
# add mode to list
client.monitors[0].modes += new_mode
# or add multiple
client.monitors[0].modes += [Mode(), Mode()]
# delete a mode we don't want
del client.monitors[0].modes[0]

#
# Refresh Rates
#

# add a refresh rate
client.monitors[0].modes[0].modes[0].refresh_rates += 90

# add multiple refresh rates
client.monitors[0].modes[0].modes[0].refresh_rates += [90, 120, 240]

# delete a refresh rate
del client.monitors[0].modes[0].modes[0].refresh_rates[0]

# set a refresh rate
client.monitors[0].modes[0].modes[0].refresh_rates[0] = 90

#
# DriverClient functions
#

# get the id of Monitor belonging to name or id
#
# DriverClient.find_id(query: str) -> Optional[int]
client.find_id("myname")

# get a Monitor by id
#
# DriverClient.find_monitor(int) -> Optional[Monitor]
client.find_monitor(5)

# get a Monitor by name or id
#
# DriverClient.find_monitor_query(query: str) -> Optional[Monitor]
client.find_monitor_query("name")

# Get the closest available free ID. Note that if internal state is stale, this may result in a duplicate ID
# which the driver will ignore when you notify it of changes
#
# DriverClient.new_id(id: Optional[int] = None) -> Optional[int]
client.new_id()
# you can ask for a preferred id, and it'll give it to you if available.
# if the id you asked for is a duplicate, None gets returned
client.new_id(5)

# send changes to driver. all changes are done in-memory until you notify
client.notify()

# save (persist) current in-memory changes to user across reboots
client.persist()

# if any other clients elsewhere modify client while your script is running
# you can ask to be notified.
# this represents the complete current state of the driver
#
# DriverClient.receive(Callable[list[Monitor], None])
client.receive(lambda d: print(d))
# one way to use this might be to auto update your driver instance
def set_monitors(data):
    client.monitors = data
client.receive(set_monitors)

# gets latest states from driver
#
# DriverClient.get_state() -> list[Monitor]
client.get_state()

# remove monitors by id
#
# DriverClient.remove(list[int])
client.remove([1,2,3])

# set enable status on monitors by id
#
# DriverClient.set_enabled(list[int], bool)
client.set_enabled([1,2,3], true)

# set enable status on monitors by query
#
# DriverClient.set_enabled_query(list[str], bool)
client.set_enabled_query(["name1", "name2", "name3"], true)
