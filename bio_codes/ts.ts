import { Developer } from 'github.com/Developer'
import { Skills } from 'github.com/Skills'

class Bio extends Developer, Skills {
    name: string = 'Rafael'
    age: number = 20
    location: string = 'Curitiba, PR - Brazil'
    languages: string[] = [
        'JavaScript', 'TypeScript', 'Rust', 'Java', 'C++', 'Python'
    ]

    constructor() {
        super()
        this.skills = [
            'Frontend Development', 
            'Backend Development',
            'Studying all the time'
        ]
    }
}