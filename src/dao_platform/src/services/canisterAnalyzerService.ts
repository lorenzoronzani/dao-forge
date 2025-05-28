import { Actor } from "@dfinity/agent";

export interface MethodSignature {
    name: string;
    parametersType: string[];
    returnType: string;
}

export class CanisterAnalyzerService {
    static extractMethods(actor: any): MethodSignature[] {
        const methods: MethodSignature[] = [];

        // Get all method names from the actor
        const methodNames = this.getMethodNames(actor);

        methodNames.forEach(methodName => {
            const signature = this.extractMethodSignature(actor, methodName);
            if (signature) {
                methods.push(signature);
            }
        });

        return methods;
    }

    private static getMethodNames(actor: any): string[] {
        return Object.getOwnPropertyNames(actor).filter(name =>
            typeof actor[name] === 'function' && !name.startsWith('_')
        );
    }

    private static extractMethodSignature(actor: any, methodName: string) {
        const method = Actor.interfaceOf(actor)._fields.find(field => field[0] === methodName);

        if (!method) {
            return;
        }

        const args = method[1].argTypes;
        const result = method[1].retTypes;

        const parametersType = args.map(arg => this.candidTypeToTypeScript(arg));
        const returnType = this.candidTypeToTypeScript(result);

        return {
            name: methodName,
            parametersType,
            returnType
        };
    }

    private static candidTypeToTypeScript(type: any): string {
        switch (type.name) {
            case 'text':
                return 'string';
            case 'nat32':
                return 'number';
            case undefined:
                return type[0].name;
            default:
                return 'unknown';
        }
    }
}
export default CanisterAnalyzerService;