class Bio():
    name = 'Rafael'
    age = 20
    location = 'Curitiba, PR - Brazil'
    languages = [
        'JavaScript', 'TypeScript', 'Rust', 'Java', 'C++', 'Python'
    ]
    skills = [
        'Frontend Development',
        'Backend Development',
        'Studying all the time'
    ]

    def __init__(self):
        print('Hello, my name is', self.name)
        print('I am', self.age, 'years old')
        print('I live in', self.location)
        print('I am a developer and I have the following skills:')
        for skill in self.skills:
            print(' -', skill)
        print('I am also knowledgeable in the following languages:')
        for language in self.languages:
            print(' -', language)


Bio()
