import Developer;
import Skills;

public class Bio implements Developer, Skills {

  private String name = "Rafael";
  private int age = 20;
  private String location = "Curitiba, PR - Brazil";
  private String[] languages = {
    "JavaScript",
    "TypeScript",
    "Rust",
    "Java",
    "C++",
    "Python",
  };
  private String[] skills = {
    "Frontend Development",
    "Backend Development",
    "Studying all the time",
  };

  public Bio() {
    System.out.println("Hello, my name is " + this.name);
    System.out.println("I am " + this.age + " years old");
    System.out.println("I live in " + this.location);
    System.out.println("I am a developer and I have the following skills:");
    for (String skill : this.skills) {
      System.out.println(" - " + skill);
    }
    System.out.println("I am also knowledgeable in the following languages:");
    for (String language : this.languages) {
      System.out.println(" - " + language);
    }
  }

  public static void main(String[] args) {
    new Bio();
  }
}
