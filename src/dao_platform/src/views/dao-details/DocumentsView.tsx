import { Card, CardContent, CardDescription, CardHeader, CardTitle } from "@/components/ui/card";
import { File } from "lucide-react";

export const DocumentsView = () => {
    return (
        <Card>
            <CardHeader>
                <CardTitle>Documents</CardTitle>
                <CardDescription>Important DAO documents and files</CardDescription>
            </CardHeader>
            <CardContent>
                <div className="text-center py-8 text-slate-500">
                    <File className="h-12 w-12 mx-auto mb-4 text-slate-300" />
                    <h3 className="font-medium mb-1">No Documents Yet</h3>
                    <p>There are no documents available for this DAO.</p>
                </div>
            </CardContent>
        </Card>
    );
};