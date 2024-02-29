import { Button, Form, Input, Space, Table } from "antd";
import BasicLayout from "../components/layout";
import useMessage from "antd/es/message/useMessage";
import { useEffect, useState } from "react";
import { Course, ExportUsersConfig, User } from "../lib/model";
import { invoke } from "@tauri-apps/api";
import CourseSelect from "../components/course_select";

export default function UsersPage() {
    const [messageApi, contextHolder] = useMessage();
    const [operating, setOperating] = useState<boolean>(false);
    // const [onlyExportStudents, setOnlyExportStudents] = useState<boolean>(true);
    const [courses, setCourses] = useState<Course[]>([]);
    const [users, setUsers] = useState<User[]>([]);
    const [selectedUsers, setSelectedUsers] = useState<User[]>([]);
    // const [selectedCourseId, setSelectedCourseId] = useState<number>(-1);
    const [form] = Form.useForm<ExportUsersConfig>();
    useEffect(() => {
        initCourses();
        form.setFieldsValue({ save_name: "用户名单" } as ExportUsersConfig)
    }, []);

    const handleGetUsers = async (courseId: number) => {
        if (courseId === -1) {
            return;
        }
        setOperating(true);
        try {
            let users = await invoke("list_course_users", { courseId }) as User[];
            users.map(user => user.key = user.id);
            setUsers(users);
        } catch (e) {
            messageApi.error(e as string);
        }
        setOperating(false);
    }

    const columns = ['id', 'name', 'email', 'created_at', 'sortable_name', 'short_name', 'login_id'].map(column => ({
        title: column,
        dataIndex: column,
        key: column,
    }));

    const initCourses = async () => {
        try {
            let courses = await invoke("list_courses") as Course[];
            setCourses(courses);
        } catch (e) {
            messageApi.error(e as string);
        }
    }

    const handleCourseSelect = async (selected: string) => {
        let selectedCourse = courses.find(course => course.name === selected);
        if (selectedCourse) {
            // setSelectedCourseId(selectedCourse.id);
            handleGetUsers(selectedCourse.id);
        }
    }

    const handleSelected = (_: React.Key[], selectedUsers: User[]) => {
        setSelectedUsers(selectedUsers);
    }

    const handleExport = async (config: ExportUsersConfig) => {
        try {
            await invoke("export_users", { users: selectedUsers, saveName: config.save_name + '.xlsx' });
            messageApi.success("导出成功！🎉", 0.5);
        } catch (e) {
            messageApi.error(e as string);
        }
    }

    return <BasicLayout>
        {contextHolder}
        <Space direction="vertical" style={{ width: "100%", overflow: "scroll" }} size={"large"}>
            <CourseSelect onChange={handleCourseSelect} disabled={operating} courses={courses} />
            <Table style={{ width: "100%" }}
                columns={columns}
                dataSource={users}
                pagination={false}
                rowSelection={{ onChange: handleSelected }}
            />
            <Form
                form={form}
                layout="vertical"
                onFinish={handleExport}
                preserve={false}
            >
                <Form.Item name="save_name" label="导出文件名（无需扩展名）">
                    <Input placeholder="请输入导出文件名（无需扩展名）" />
                </Form.Item>
                <Form.Item>
                    <Button disabled={operating} type="primary" htmlType="submit">
                        导出
                    </Button>
                </Form.Item>
            </Form>
        </Space>
    </BasicLayout>
}