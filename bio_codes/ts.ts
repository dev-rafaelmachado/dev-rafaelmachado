import { Developer } from 'github.com/Developer'
import { Skills } from 'github.com/Skills'
class Bio implements Developer, Skills{
    name: string = 'Rafael'
    age: number = 20
    location: string = 'Curitiba, PR - Brazil'
    languages: string[] = [
        'JavaScript', 'TypeScript', 'Rust', 'Java', 'C++', 'Python'
    ]
    skills = [
        'Frontend Development', 
        'Backend Development',
        'Studying all the time'
    ]

    constructor() {
        console.log('Hello, my name is', this.name)
        console.log('I am', this.age, 'years old')
        console.log('I live in', this.location)
        console.log('I am a developer and I have the following skills:')
        this.skills.forEach(skill => console.log(' -', skill))
        console.log('I am also knowledgeable in the following languages:')
        this.languages.forEach(language => console.log(' -', language))
    }
}

new Bio()