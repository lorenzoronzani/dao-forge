import { Card, CardHeader, CardTitle, CardDescription, CardContent } from "@/components/ui/card";
import { File } from "lucide-react";

export const PublicationsView = () => {
    return (
        <Card>
            <CardHeader>
                <CardTitle>SOGC Publications</CardTitle>
                <CardDescription>Official publications in the Swiss Official Gazette of Commerce</CardDescription>
            </CardHeader>
            <CardContent>
                <div className="text-center py-8 text-slate-500">
                    <File className="h-12 w-12 mx-auto mb-4 text-slate-300" />
                    <h3 className="font-medium mb-1">No SOGC Publications</h3>
                    <p>There are no publications in the Swiss Official Gazette of Commerce for this DAO.</p>
                </div>

            </CardContent>
        </Card>
    )
};
