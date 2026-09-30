package com.sicpa.jsgxannotations;

import java.io.IOException;
import java.io.Writer;
import java.util.Set;

import javax.annotation.processing.*;

import javax.lang.model.SourceVersion;
import javax.lang.model.element.Element;
import javax.lang.model.element.TypeElement;
import javax.tools.Diagnostic;
import javax.tools.JavaFileObject;

import com.google.auto.service.AutoService;

@SupportedAnnotationTypes({"com.sicpa.jsgxannotations.TrustedApplication"})
@SupportedSourceVersion(SourceVersion.RELEASE_21)
@AutoService(AnnotationProcessor.class)
public class AnnotationProcessor extends AbstractProcessor {

	@Override
	public synchronized void init(ProcessingEnvironment processingEnv) {
		super.init(processingEnv);
	}

	@Override
	protected synchronized boolean isInitialized() {
		return super.isInitialized();
	}

	@Override
	public boolean process(Set<? extends TypeElement> annotations, RoundEnvironment roundEnv) {
		for (TypeElement annotation : annotations) {
			// Find elements annotated with MyCustomAnnotation
			for (Element element : roundEnv.getElementsAnnotatedWith(annotation)) {
				// Process each element
				Filer filer = processingEnv.getFiler();
				try {
					JavaFileObject fileObject = filer.createSourceFile("com.sicpa.javata.Main");
					var className = element.getSimpleName();
					var classn = ((TypeElement) element).getQualifiedName();
					processingEnv.getMessager().printMessage(Diagnostic.Kind.NOTE, "Running " + classn.toString());
					try (Writer writer = fileObject.openWriter()) {
						writer.write("package com.sicpa.javata;\n");
						writer.write(String.format("import %s;\n", classn));
						writer.write("public class Main {\n");
						writer.write("public static void main(String[] argv) {\n");
						writer.write(String.format("%s ta = new %s();\n",className, className));
						writer.write("IPCListener listener = new IPCListener(ta);\n");
						writer.write("listener.run();\n");
						writer.write("}\n}\n");
					}
				} catch (IOException e) {
					processingEnv.getMessager().printMessage(Diagnostic.Kind.ERROR,
							"Failed to generate class: " + e.getMessage());
				}

			}
		}
		return true; // No further processing of this annotation type
	}

}
