import { MethodSignature, Parameter } from "@/services/canisterAnalyzerService";
import { Code } from "lucide-react";

interface MethodInformationSectionProps {
    method: MethodSignature;
}


export const MethodInformationSection = ({ method }: MethodInformationSectionProps) => {
    const formatParams = (params: Parameter[]): string => {
        const formattedParams = params.map((param) => {
            if (param.type === 'record') {
                return `${param?.name || 'param'}: record { ${param.fields?.map((field) => `${field.name}: ${field.type}`).join(', ')} }`;
            }

            return `${param?.name || 'param'}: ${param.type}`;
        });
        return formattedParams.join(', ');
    }

    return (
        <div className="p-3 bg-slate-50 rounded-lg">
            <div className="flex items-center gap-2 mb-2">
                <Code className="h-4 w-4" />
                <span className="font-medium text-sm">Method Signature</span>
            </div>
            <code className="text-xs bg-white p-2 rounded border block">
                {method.name}({formatParams(method.parameters)}) → {method.returnType}
            </code>
        </div>
    );
}

export default MethodInformationSection;

