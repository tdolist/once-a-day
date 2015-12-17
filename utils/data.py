import json
import os

# file paths for the settings and the todolist file
home = os.environ['HOME']
jsonfile = '{home}/.oad/data.json'.format(home=home)


def load_mail():
    return load()['mail']


def load():
    try:
        with open(jsonfile, 'r') as backupfile:
            return json.load(backupfile)
    except FileNotFoundError:
        os.makedirs(os.path.dirname(jsonfile), exist_ok=True)
        # TODO: initdict needs a lookup
        initdict = {'mail': {}, 'notifications': {}}
        with open(jsonfile, 'w') as f:
            json.dump(initdict, f)
        return initdict
