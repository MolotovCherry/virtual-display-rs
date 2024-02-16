from pyvdd import Monitors

#
# Monitor functionality
#
mons = Monitors()
print(mons)
# [Monitor { id: 1, name: Some("bar"), enabled: true, modes: [Mode { width: 1920, height: 1080, refresh_rates: [90, 120] }] }]

# you can get any monitor by the monitors specific ID
mon1 = mons[1]
print(mon1)
# Monitor { id: 1, name: Some("bar"), enabled: true, modes: [Mode { width: 1920, height: 1080, refresh_rates: [90, 120] }] }

# get/set properties on the monitor
mon1.id = 0
# get/set name
mon1.name = "Foo"
mon1.name = None
# set enabled status
mon1.enabled = False

print(mon1.modes)
# node, this can be set with a dict
# [Mode { width: 1920, height: 1080, refresh_rates: [90, 120] }]

# set the whole modes in one go
mon1.modes = [{
    "width": 2000,
    "height": 1000,
    "refresh_rates": [120, 90, 60]
}]

# you can iterate
for mon in mons:
    print(mon)

# set a new monitor by id, or overwrite existing monitor id if it exists
mons[1] = {
    "name": "foo",
    "enabled": True,
    "modes": [{
        "width": 2000,
        "height": 1000,
        "refresh_rates": [120, 90, 60]
    }]
}

# remove monitor
del mons[1]

# whenever any changes are made, you can notify the driver to update
mons.notify()

# you can remove specific monitors by id; this will immediately notify the driver
mons.remove([1,2,3])

# remove all monitors; this will immediately notify the driver
mons.remove_all()

#
# Modes
#
mode = mons1.modes[0]
# getter/setter prop
mode.width = 2000
# getter/setter prop
mode.height = 1000
# getter/setter prop
mode.refresh_rates = [120, 90, 60]

# set mode to new data from dict
# can only overwrite already existing elems
mon1.modes[0] = {
    "width": 2000,
    "height": 1000,
    "refresh_rates": [120, 90, 60]
}

# remove mode
del mon1.modes[0]

# you can iterate
for mode in mon1.modes:
    print(mode)

# you can add another mode
mon1.modes + {
    "width": 2000,
    "height": 1000,
    "refresh_rates": [120, 90, 60]
}

# or you can add a list of modes
mon1.modes + [{
    "width": 2000,
    "height": 1000,
    "refresh_rates": [120, 90, 60]
}]
